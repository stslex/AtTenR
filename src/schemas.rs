diesel::table! {
    users(uuid) {
        uuid -> Uuid,
        login -> Varchar,
        secret -> Text,
        username -> Varchar,
    }
}
