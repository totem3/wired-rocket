table! {
    messages (id) {
        id -> Unsigned<Bigint>,
        room_id -> Unsigned<Bigint>,
        content -> Text,
    }
}

table! {
    rooms (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    rooms,
);
