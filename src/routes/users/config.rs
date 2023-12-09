use actix_web::web;

use super::me::{me, update_me};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(me).service(update_me));
}
