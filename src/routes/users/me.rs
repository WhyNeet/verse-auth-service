use actix_web::{get, post, web, HttpResponse};
use common::server::{
    auth::{
        errors::{AuthError, AuthResult},
        token::access::AccessToken,
    },
    utils::{string_to_oid, transform_errors},
};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::repo::{users::model::UserResponse, MongoRepo};

#[get("/@me")]
pub async fn me(at: AccessToken, mongo_repo: web::Data<MongoRepo>) -> AuthResult<HttpResponse> {
    let user = mongo_repo
        .users
        .get_by_id(&string_to_oid(&at.sub)?)
        .await
        .or(Err(AuthError::Internal))?
        .ok_or(AuthError::InvalidToken)?;

    let user: UserResponse = user.into();

    Ok(HttpResponse::Ok().json(json!({ "user": user })))
}

#[derive(Deserialize, Validate)]
pub struct UserUpdatePayload {
    #[validate(length(min = 2, max = 50, message = "Name must be 2-50 characters long."))]
    pub name: Option<String>,

    #[validate(length(min = 1, max = 250, message = "Bio must be 1-250 characters long."))]
    pub bio: Option<String>,

    #[validate(length(
        min = 1,
        max = 250,
        message = "Location must be 1-250 characters long."
    ))]
    pub location: Option<String>,
}

#[post("/@me")]
pub async fn update_me(
    at: AccessToken,
    mongo_repo: web::Data<MongoRepo>,
    payload: web::Json<UserUpdatePayload>,
) -> AuthResult<HttpResponse> {
    payload
        .validate()
        .map_err(transform_errors)
        .map_err(AuthError::InvalidPayload)?;

    mongo_repo
        .users
        .update_user(
            &string_to_oid(&at.sub)?,
            payload.name.as_deref(),
            payload.bio.as_deref(),
            payload.location.as_deref(),
        )
        .await
        .or(Err(AuthError::Internal))?;

    Ok(HttpResponse::Ok().json(json!({ "message": "User updated." })))
}
