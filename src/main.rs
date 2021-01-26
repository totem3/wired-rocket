#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_multipart_form_data;
extern crate serde;
extern crate tungstenite;

use std::sync::mpsc;

use rocket_contrib::databases::diesel as db;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::handlebars::handlebars_helper;
use rocket_contrib::templates::Template;

use crate::web_socket_server::MessageChannel;

pub mod models;
pub mod routes;
pub mod schema;
pub mod web_socket_server;

#[database("chat")]
pub struct DBConnection(db::MysqlConnection);

fn main() {
    let template_fairing = Template::custom(|engine| {
        handlebars_helper!(nl2br: |v: str| {
            let lines: Vec<_> = v.lines().collect();
            lines.join("<br>")
        });
        engine.handlebars.register_helper("nl2br", Box::new(nl2br));
    });

    let (tx, rx) = mpsc::sync_channel(1024);
    web_socket_server::launch("127.0.01:8001", rx);
    rocket::ignite()
        .manage(MessageChannel(tx))
        .attach(template_fairing)
        .attach(DBConnection::fairing())
        .mount(
            "/",
            routes![
                routes::rooms::index,
                routes::rooms::create_message,
                routes::rooms::edit_room,
                routes::rooms::update_room
            ],
        )
        .mount(
            "/assets",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
        .launch();
}
