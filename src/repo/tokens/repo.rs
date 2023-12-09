use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};

use super::model::RefreshToken;

pub struct TokensRepo {
    client: Client,
}

impl TokensRepo {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl TokensRepo {
    pub async fn blacklist_token(&self, token_id: ObjectId) -> anyhow::Result<()> {
        let tokens = self.tokens();

        tokens.insert_one(RefreshToken::new(token_id), None).await?;

        Ok(())
    }

    pub async fn is_token_blacklisted(&self, token_id: ObjectId) -> anyhow::Result<bool> {
        let tokens = self.tokens();

        Ok(tokens
            .find_one(doc! { "_id": token_id }, None)
            .await?
            .is_some())
    }

    fn tokens(&self) -> Collection<RefreshToken> {
        self.client.database("auth").collection("rt_blacklist")
    }
}
