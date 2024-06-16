#[derive(Clone, Copy)]
pub struct JwtObject<'a> {
    pub uuid: &'a str,
    pub username: &'a str,
}

pub struct JwtResult {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug)]
pub enum JwtGeneratorError {
    InvalidEnvSecret,
    DurationOutOfBound,
    TimeCreationError,
    SignWithKey,
    CreateKey,
}
