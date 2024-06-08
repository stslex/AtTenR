pub struct UserAuthRequest<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
}

pub struct UserAuthResposne {
    uuid: uuid::Uuid,
    username: String,
    email: String,
    password: String,
}

pub enum UserAuthError {}
