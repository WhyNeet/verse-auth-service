use actix_web::web;

use super::{login::login, register::register, tokens::refresh_token, verify_email::verify_email};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(register)
            .service(login)
            .service(verify_email)
            .service(refresh_token),
    );
}
