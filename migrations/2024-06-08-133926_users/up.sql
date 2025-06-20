CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    google_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

create unique index IF NOT EXISTS users_uuid_uindex on users (uuid);
create unique index IF NOT EXISTS users_google_id_uindex on users (google_id);
create unique index IF NOT EXISTS users_email_uindex on users (email);
create index IF NOT EXISTS users_created_at_index on users (created_at);
create index IF NOT EXISTS users_updated_at_index on users (updated_at);
