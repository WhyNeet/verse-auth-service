use actix_web::web;

use super::{auth, users};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::configure)
            .configure(users::configure),
    );
}
