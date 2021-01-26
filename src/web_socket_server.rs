use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::spawn;

use diesel::{Connection, QueryDsl, RunQueryDsl};

use crate::models::message::Message;

pub enum Action {
    RenameRoom(u64, String),
    NewMessage(u64),
}

pub struct Broker {
    rx: Receiver<Action>,
    wss: Arc<Mutex<Vec<tungstenite::WebSocket<TcpStream>>>>,
}

impl Broker {
    pub fn write_all(&self, message: tungstenite::Message) {
        let mut wss = self.wss.lock().unwrap();
        for ws in &mut *wss {
            let _ = ws.write_message(message.clone());
        }
    }
}

pub struct MessageChannel(pub SyncSender<Action>);

pub fn launch<A: ToSocketAddrs>(addr: A, rx: Receiver<Action>) {
    let wss = Arc::new(Mutex::new(Vec::new()));
    {
        let wss = wss.clone();
        let server = TcpListener::bind(addr).unwrap();
        spawn(move || {
            for stream in server.incoming() {
                let websocket = tungstenite::accept(stream.unwrap()).unwrap();
                let mut wss = wss.lock().unwrap();
                wss.push(websocket);
            }
        });
    }
    let broker = Broker {
        rx,
        wss: wss.clone(),
    };
    spawn(move || loop {
        match broker.rx.recv() {
            Ok(Action::RenameRoom(room_id, new_name)) => {
                broker.write_all(tungstenite::Message::text(format!(r#"
                        <turbo-stream action="replace" target="room_list_{room_id}">
                            <template>
                                <a id="room_list_{room_id}" href="/?room_id={room_id}">
                                    <li class="px-4 hover:bg-red-200" data-room-id="{room_id}">
                                        {room_name}
                                    </li>
                                </a>
                            </template>
                        </turbo-stream>
                        <turbo-stream action="replace" target="room_title_{room_id}">
                            <template>
                                <h2 id="room_title_{room_id}" class="mr-4">
                                    {room_name}
                                </h2>
                            </template>
                        </turbo-stream>
                        "#, room_id = room_id, room_name = new_name)));
            }
            Ok(Action::NewMessage(message_id)) => {
                use crate::schema::messages;
                let url = "mysql://root:root@127.0.0.1:3306/chat";
                let conn = diesel::MysqlConnection::establish(url).unwrap();
                let message: Message = messages::table.find(message_id).first(&conn).unwrap();
                broker.write_all(tungstenite::Message::text(format!(
                    r#"
                        <turbo-stream action="append" target="messages_{room_id}">
                            <template>
                                <li class="p-4">
                                    {content}
                                </li>
                            </template>
                        </turbo-stream>
                        "#,
                    room_id =  message.room_id, content = message.content
                )));
            }
            _ => {}
        }
    });
}
