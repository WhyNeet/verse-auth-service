use actix_web::{middleware::Logger, web, App, HttpServer};
use common::{
    server::{auth::jwt::JwtAuth, logging, routes as common_routes},
    utils,
};

use auth_service::{mailing::mailer::Mailer, repo::MongoRepo, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().expect("failed to read .env in debug");
    logging::init_logger().expect("failed to initialize logger");

    let jwt_auth = JwtAuth::new_from_env().expect("failed to initialize JwtAuth from env");
    let jwt_auth = web::Data::new(jwt_auth);

    let mongo_repo = MongoRepo::new_from_env()
        .await
        .expect("failed to initialize Db connection");
    let mongo_repo = web::Data::new(mongo_repo);

    let mailer = Mailer::new_from_env().expect("failed to initialize Mailer from env");
    let mailer = web::Data::new(mailer);

    HttpServer::new(move || {
        App::new()
            .service(common_routes::healthcheck)
            .app_data(jwt_auth.clone())
            .app_data(mongo_repo.clone())
            .app_data(mailer.clone())
            .configure(routes::configure)
            .wrap(Logger::default())
    })
    .bind((
        "0.0.0.0",
        utils::get_server_port().expect("wrong PORT environment variable found"),
    ))?
    .run()
    .await
}
