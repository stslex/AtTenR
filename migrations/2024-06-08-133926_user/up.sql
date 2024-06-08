-- Your SQL goes here
create table IF NOT EXISTS user
(
    uuid uuid default uuid_generate_v4() not null constraint table_name_pk primary key,
    login varchar(123) not null,
    secret text not null,
    username varchar(128) not null
);

create unique index IF NOT EXISTS user_uuid_uindex on users (uuid);
create unique index IF NOT EXISTS user_login_uindex on users (login);
create unique index IF NOT EXISTS user_username_uindex on users (username);
create index IF NOT EXISTS users_secret_uindex on users (secret);