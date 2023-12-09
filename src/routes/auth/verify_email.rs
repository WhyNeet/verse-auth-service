use std::str::FromStr;

use actix_web::{get, web, HttpResponse};
use common::server::auth::{
    errors::{AuthError, AuthResult},
    jwt::JwtAuth,
    token::access::AccessToken,
};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use serde_json::json;

use crate::repo::MongoRepo;

#[derive(Deserialize)]
pub struct Token {
    token: String,
}

#[get("/verify_email")]
pub async fn verify_email(
    token: web::Query<Token>,
    jwt_auth: web::Data<JwtAuth>,
    mongo_repo: web::Data<MongoRepo>,
) -> AuthResult<HttpResponse> {
    let token_data = jwt_auth
        .decode_jwt::<AccessToken>(&token.token)
        .or(Err(AuthError::InvalidToken))?;
    let user_id = ObjectId::from_str(&token_data.sub).or(Err(AuthError::InvalidToken))?;

    mongo_repo
        .users
        .verify_email(user_id)
        .await
        .or(Err(AuthError::Internal))?;

    Ok(HttpResponse::Ok().json(json!({ "message": "Email verified." })))
}
