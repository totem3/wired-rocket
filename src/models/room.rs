use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Room {
    pub id: u64,
    pub name: String,
}
