use serde::Serialize;

use crate::schema::rooms;

#[derive(Debug, Queryable, Serialize)]
pub struct Room {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Insertable, Serialize)]
#[table_name = "rooms"]
pub struct NewRoom {
    pub name: String,
}
