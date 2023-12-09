pub mod tokens;
pub mod users;

use std::env;

use log::info;
use mongodb::{bson::doc, options::ClientOptions, Client};

use self::{tokens::repo::TokensRepo, users::UsersRepo};

pub struct MongoRepo {
    pub users: UsersRepo,
    pub tokens: TokensRepo,
}

impl MongoRepo {
    pub async fn new(db_uri: &str) -> anyhow::Result<Self> {
        info!("Establishing database connection");

        let client_options = ClientOptions::parse(db_uri).await?;
        let client = Client::with_options(client_options)?;

        Self::check_connection(&client).await?;

        info!("Database connection established");

        let users = UsersRepo::new(client.clone());
        let tokens = TokensRepo::new(client);

        Ok(Self { users, tokens })
    }

    pub async fn new_from_env() -> anyhow::Result<Self> {
        let db_uri = env::var("MONGODB_URI")?;

        Self::new(&db_uri).await
    }

    async fn check_connection(client: &Client) -> anyhow::Result<()> {
        client
            .database("auth")
            .run_command(doc! { "ping": 1 }, None)
            .await?;

        Ok(())
    }
}
