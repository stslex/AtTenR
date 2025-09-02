CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table IF NOT EXISTS todo
(
    uuid UUID DEFAULT uuid_generate_v4() not NULL constraint table_todo_pk primary key,
    user_uuid UUID not NULL,
    title VARCHAR(128) not NULL,
    description VARCHAR(128) not NULL,
    status VARCHAR(128) not NULL,
    created_at BIGINT not NULL,
    updated_at BIGINT not NULL,
    expires_at BIGINT not NULL
);

create unique index IF NOT EXISTS todo_uuid_uindex on todo (uuid);
create index IF NOT EXISTS todo_user_uuid_uindex on todo (user_uuid);
create index IF NOT EXISTS todo_title_uindex on todo (title);
create index IF NOT EXISTS todo_description_uindex on todo (description);
create index IF NOT EXISTS todo_status_uindex on todo (status);
create index IF NOT EXISTS todo_created_at_uindex on todo (created_at);
create index IF NOT EXISTS todo_updated_at_uindex on todo (updated_at);
create index IF NOT EXISTS todo_expires_at_uindex on todo (expires_at);
