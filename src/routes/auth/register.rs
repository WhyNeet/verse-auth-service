use actix_web::{post, web, HttpRequest, HttpResponse};
use chrono::Duration;
use lettre::message::header::ContentType;
use serde_json::json;
use validator::Validate;

use crate::{
    mailing::mailer::Mailer,
    repo::{
        users::{
            model::{User, UserResponse},
            payload::UserCreatePayload,
        },
        MongoRepo,
    },
};
use common::server::{
    auth::{
        errors::{AuthError, AuthResult},
        jwt::JwtAuth,
        token::access::AccessToken,
    },
    utils,
};

#[post("/register")]
pub async fn register(
    mailer: web::Data<Mailer>,
    payload: web::Json<UserCreatePayload>,
    jwt_auth: web::Data<JwtAuth>,
    mongo_repo: web::Data<MongoRepo>,
    request: HttpRequest,
) -> AuthResult<HttpResponse> {
    payload
        .validate()
        .map_err(utils::transform_errors)
        .map_err(AuthError::InvalidPayload)?;

    if mongo_repo
        .users
        .exists_by_email(&payload.email)
        .await
        .or(Err(AuthError::Internal))?
    {
        return Err(AuthError::UserAlreadyExists("email"));
    }

    if mongo_repo
        .users
        .exists_by_username(&payload.username)
        .await
        .or(Err(AuthError::Internal))?
    {
        return Err(AuthError::UserAlreadyExists("username"));
    }

    let user = User::from_create_payload(payload.0).unwrap();
    mongo_repo
        .users
        .create(&user)
        .await
        .or(Err(AuthError::Internal))?;

    let host = {
        let conn_info = request.connection_info();
        let host = String::from(conn_info.host());
        host
    };

    let token = AccessToken::new(user._id.to_string(), Duration::minutes(10));
    let token = jwt_auth.encode_jwt(token).unwrap();

    mailer.send(&user.email, "Postland", "Verify your email", format!(r#"<h1>Complete sign-up.</h1><a href="http{}://{host}/api/auth/verify_email?token={token}">Click here to verify email.</a><p>Link is valid for 10 minutes.</p>"#, if cfg!(debug_assertions) { "" } else { "s" }), Some(ContentType::TEXT_HTML)).await.or(Err(AuthError::Internal))?;

    let (at, rt) = jwt_auth
        .generate_tokens(user._id.to_hex())
        .or(Err(AuthError::Internal))?;

    let at_cookie = JwtAuth::generate_cookie(&at);
    let user: UserResponse = user.into();

    Ok(HttpResponse::Ok()
        .cookie(at_cookie)
        .json(json!({ "user": user, "rt": rt })))
}
