use crate::repo::users::{model::UserResponse, payload::RE_USERNAME};
use actix_web::{post, web, HttpResponse};
use common::{
    hashing,
    server::{
        auth::{
            errors::{AuthError, AuthResult},
            jwt::JwtAuth,
        },
        utils,
    },
};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::repo::MongoRepo;

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email(message = "Invalid email."))]
    email: Option<String>,

    #[validate(regex(path = "RE_USERNAME", message = "Invalid username."))]
    username: Option<String>,

    #[validate(length(min = 8, max = 72, message = "Password must be 8-72 characters long."))]
    password: String,
}

#[post("/login")]
pub async fn login(
    jwt_auth: web::Data<JwtAuth>,
    mongo_repo: web::Data<MongoRepo>,
    payload: web::Json<LoginPayload>,
) -> AuthResult<HttpResponse> {
    payload
        .validate()
        .map_err(utils::transform_errors)
        .map_err(AuthError::InvalidPayload)?;

    if payload.email.is_none() && payload.username.is_none() {
        return Err(AuthError::InvalidCredentials);
    }

    let user = if let Some(email) = payload.email.as_ref() {
        mongo_repo.users.get_by_email(email).await
    } else if let Some(username) = payload.username.as_ref() {
        mongo_repo.users.get_by_username(username).await
    } else {
        return Err(AuthError::InvalidCredentials);
    }
    .or(Err(AuthError::Internal))?
    .ok_or(AuthError::UserDoesNotExist(if payload.email.is_some() {
        "email"
    } else {
        "username"
    }))?;

    hashing::verify_password(payload.password.as_bytes(), user.get_password_hash())
        .or(Err(AuthError::WrongPassword))?;

    let (at, rt) = jwt_auth
        .generate_tokens(user._id.to_hex())
        .or(Err(AuthError::Internal))?;

    let at_cookie = JwtAuth::generate_cookie(&at);
    let user: UserResponse = user.into();

    Ok(HttpResponse::Ok()
        .cookie(at_cookie)
        .json(json!({ "user": user, "rt": rt })))
}
