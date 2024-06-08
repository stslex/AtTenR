use uuid::Uuid;

pub struct UserEntityCreate<'a> {
    uuid: Uuid,
    name: &'a str,
    email: &'a str,
}

pub struct UserEntity {
    id: i32,
    name: String,
    email: String,
}

pub enum UserDatabaseError {}
