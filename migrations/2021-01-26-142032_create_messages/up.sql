-- Your SQL goes here
create table messages (
    id bigint unsigned auto_increment primary key,
    room_id bigint unsigned not null,
    content text not null,
    index `idx_messages_on_room_id`(room_id)
)