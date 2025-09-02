diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        google_id -> Text,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    todo (uuid){
        uuid -> Uuid,
        user_uuid -> Uuid,
        title -> Varchar,
        description -> Varchar,
        status -> Varchar,
        created_at -> Int8,
        updated_at -> Int8,
        expires_at -> Int8,
    }
}
