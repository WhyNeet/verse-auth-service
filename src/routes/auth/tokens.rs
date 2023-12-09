use std::str::FromStr;

use actix_web::{post, web, HttpResponse};
use common::server::auth::{
    errors::{AuthError, AuthResult},
    jwt::JwtAuth,
    token::refresh::RefreshToken,
};
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::repo::MongoRepo;

#[post("/tokens/refresh")]
pub async fn refresh_token(
    rt: RefreshToken,
    mongo_repo: web::Data<MongoRepo>,
    jwt_auth: web::Data<JwtAuth>,
) -> AuthResult<HttpResponse> {
    let token_id = ObjectId::from_str(&rt.sub).or(Err(AuthError::InvalidToken))?;
    if mongo_repo
        .tokens
        .is_token_blacklisted(token_id)
        .await
        .or(Err(AuthError::Internal))?
    {
        return Err(AuthError::InvalidToken);
    }

    let at = jwt_auth.generate_at(rt.uuid).or(Err(AuthError::Internal))?;
    let at_cookie = JwtAuth::generate_cookie(&at);

    Ok(HttpResponse::Ok()
        .cookie(at_cookie)
        .json(json!({ "message": "Token refreshed." })))
}
