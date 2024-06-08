diesel::table! {
    user(uuid) {
        uuid -> Uuid,
        login -> Varchar,
        secret -> Text,
        username -> Varchar,
    }
}
