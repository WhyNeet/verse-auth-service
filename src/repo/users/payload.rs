use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

lazy_static! {
    pub static ref RE_USERNAME: Regex = Regex::new("^[a-z0-9_-]{2,32}$").unwrap();
}

#[derive(Deserialize, Validate)]
pub struct UserCreatePayload {
    #[validate(regex(path = "RE_USERNAME", message = "Invalid username."))]
    pub username: String,

    #[validate(email(message = "Invalid email."))]
    pub email: String,

    #[validate(length(min = 2, max = 50, message = "Name must be 2-50 characters long."))]
    pub name: Option<String>,

    #[validate(length(min = 8, max = 72, message = "Password must be 8-72 characters long."))]
    pub password: String,
}
