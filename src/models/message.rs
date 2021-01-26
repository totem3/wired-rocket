use serde::Serialize;

use crate::schema::messages;

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Message {
    pub id: u64,
    pub room_id: u64,
    pub content: String,
}

#[derive(Debug, Insertable, Serialize)]
#[table_name = "messages"]
pub struct NewMessage {
    pub room_id: u64,
    pub content: String,
}

#[derive(FromForm)]
pub struct MessageForm {
    pub room_id: u64,
    pub content: String,
}
