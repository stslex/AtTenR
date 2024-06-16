use super::objects::JwtObject;

pub struct AccessToken<'a> {
    pub uuid: &'a str,
    pub username: &'a str,
}

impl<'a> Into<AccessToken<'a>> for JwtObject<'a> {
    fn into(self) -> AccessToken<'a> {
        AccessToken {
            uuid: self.uuid,
            username: self.username,
        }
    }
}

pub struct RefreshToken<'a> {
    pub uuid: &'a str,
    pub username: &'a str,
}

impl<'a> Into<RefreshToken<'a>> for JwtObject<'a> {
    fn into(self) -> RefreshToken<'a> {
        RefreshToken {
            uuid: self.uuid,
            username: self.username,
        }
    }
}

pub struct TokenGenerationModel<'a> {
    pub env_secret: &'a [u8],
    pub exp_days: i64,
    pub uuid: &'a str,
    pub username: &'a str,
}
