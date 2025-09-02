use std::env;

use actix_web::guard::{self, Guard};
use once_cell::sync::Lazy;

const API_KEY_HEADER: &str = "X-Api-Key";
const API_KEY_ENV_VAR: &str = "API_KEY";

static API_KEY: Lazy<String> =
    Lazy::new(|| env::var(API_KEY_ENV_VAR).expect("missing API key env var"));

pub fn api_key_guard() -> impl Guard {
    guard::Header(API_KEY_HEADER, API_KEY.as_str())
}
