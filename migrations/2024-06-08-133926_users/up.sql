CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table IF NOT EXISTS users
(
    uuid UUID DEFAULT uuid_generate_v4() not NULL constraint table_name_pk primary key,
    login VARCHAR(128) not NULL,
    username VARCHAR(128) not NULL,
    secret VARCHAR(128) not NULL
);

create unique index IF NOT EXISTS users_uuid_uindex on users (uuid);
create unique index IF NOT EXISTS users_login_uindex on users (login);
create unique index IF NOT EXISTS users_username_uindex on users (username);
create index IF NOT EXISTS users_secret_uindex on users (secret);
