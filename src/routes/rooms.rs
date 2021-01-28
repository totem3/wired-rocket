use rocket::http::ContentType;
use rocket::response::Redirect;
use rocket::{Data, State};
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl};
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use serde::Serialize;

use crate::diesel::RunQueryDsl;
use crate::models::message::{Message, NewMessage};
use crate::web_socket_server::{Action, MessageChannel};
use crate::DBConnection;
use crate::models::room::{Room, NewRoom};

#[derive(Debug, Serialize)]
struct RoomIndexContext {
    room_contexts: Vec<RoomContext>,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct RoomContext {
    pub room: Room,
    pub current: bool,
}

#[derive(Debug, Serialize)]
struct RoomEditContext {
    pub room: Room,
}

#[derive(Debug, Serialize)]
struct CreateMessageContext {
    pub new_message: NewMessage,
}

#[get("/?<room_id>")]
pub fn index(conn: DBConnection, room_id: Option<u64>) -> Template {
    use crate::schema::messages;
    use crate::schema::rooms;
    let rooms: Vec<Room> = rooms::table.load::<Room>(&conn.0).unwrap();
    let mut room_contexts: Vec<RoomContext> = rooms
        .into_iter()
        .map(|room| {
            let current = room_id.map(|rid| rid == room.id).unwrap_or(false);
            RoomContext { room, current }
        })
        .collect();
    let mut messages = vec![];
    if let Some(room_id) = room_id {
        messages = messages::table
            .filter(messages::room_id.eq(room_id))
            .load::<Message>(&conn.0)
            .unwrap();
    } else {
        if room_contexts.len() > 0 {
            room_contexts[0].current = true;
            messages = messages::table
                .filter(messages::room_id.eq(room_contexts[0].room.id))
                .load::<Message>(&conn.0)
                .unwrap();
        }
    }
    let context = RoomIndexContext {
        room_contexts,
        messages,
    };
    Template::render("rooms/index", &context)
}

#[get("/rooms/new")]
pub fn new_room() -> Template {
    Template::render("rooms/new", ())
}

#[post("/rooms", format = "multipart", data = "<data>")]
pub fn create_room(
    channel: State<MessageChannel>,
    conn: DBConnection,
    content_type: &ContentType,
    data: Data,
) -> Redirect {
    use crate::schema::rooms;
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("name"),
    ]);
    let mut message_form = MultipartFormData::parse(content_type, data, options).unwrap();
    let name = message_form.texts.remove("name").unwrap()[0].text.clone();
    let new_room = NewRoom { name };
    diesel::insert_into(rooms::table)
        .values(&new_room)
        .execute(&conn.0)
        .expect("failed to create a new room");

    let new_room_id = diesel::select(last_insert_id).first(&conn.0).unwrap();

    let tx = channel.0.clone();
    let _ = tx.send(Action::NewRoom(new_room_id));

    Redirect::found(uri! {index: room_id = _ })
}

no_arg_sql_function!(
    last_insert_id,
    diesel::types::Unsigned<diesel::types::Bigint>
);

#[post("/messages", format = "multipart", data = "<data>")]
pub fn create_message(
    channel: State<MessageChannel>,
    conn: DBConnection,
    content_type: &ContentType,
    data: Data,
) -> Redirect {
    use crate::schema::messages;
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("room_id"),
        MultipartFormDataField::text("content"),
    ]);
    let mut message_form = MultipartFormData::parse(content_type, data, options).unwrap();
    let room_id = message_form.texts.remove("room_id").unwrap()[0]
        .text
        .parse::<u64>()
        .unwrap();
    let content = message_form.texts.remove("content").unwrap()[0]
        .text
        .clone();
    let new_message = NewMessage { room_id, content };
    diesel::insert_into(messages::table)
        .values(&new_message)
        .execute(&conn.0)
        .expect("failed to create a new message");

    let new_message_id = diesel::select(last_insert_id).first(&conn.0).unwrap();

    let tx = channel.0.clone();
    let _ = tx.send(Action::NewMessage(new_message_id));

    Redirect::found(uri! {index: room_id = room_id})
}

#[get("/rooms/<room_id>/edit")]
pub fn edit_room(conn: DBConnection, room_id: u64) -> Template {
    use crate::schema::rooms;
    let room = rooms::table.find(room_id).first(&conn.0).unwrap();
    let room_edit_context = RoomEditContext { room };
    Template::render("rooms/edit", &room_edit_context)
}

#[post("/rooms/<room_id>", format = "multipart", data = "<data>")]
pub fn update_room(
    channel: State<MessageChannel>,
    conn: DBConnection,
    content_type: &ContentType,
    room_id: u64,
    data: Data,
) -> Redirect {
    use crate::schema::rooms;
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("name"),
    ]);
    let mut room_form = MultipartFormData::parse(content_type, data, options).unwrap();
    let content = room_form.texts.remove("name").unwrap()[0].text.clone();
    diesel::update(rooms::dsl::rooms.filter(rooms::id.eq(room_id)))
        .set(rooms::name.eq(content.clone()))
        .execute(&conn.0)
        .unwrap();
    let tx = channel.0.clone();
    let _ = tx.send(Action::RenameRoom(room_id, content.clone()));
    Redirect::found(uri![index: room_id = room_id])
}
