use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};

use super::model::User;

pub struct UsersRepo {
    client: Client,
}

impl UsersRepo {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    fn users(&self) -> Collection<User> {
        self.client.database("auth").collection("users")
    }
}

impl UsersRepo {
    pub async fn get_by_id(&self, id: &ObjectId) -> anyhow::Result<Option<User>> {
        let users = self.users();

        let user = users.find_one(doc! { "_id": id }, None).await?;

        Ok(user)
    }

    pub async fn get_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let users = self.users();

        let user = users.find_one(doc! { "email": email }, None).await?;

        Ok(user)
    }

    pub async fn get_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        let users = self.users();

        let user = users.find_one(doc! { "username": username }, None).await?;

        Ok(user)
    }
}

impl UsersRepo {
    pub async fn exists_by_email(&self, email: &str) -> anyhow::Result<bool> {
        let users = self.users();

        Ok(users.count_documents(doc! { "email": email }, None).await? != 0)
    }

    pub async fn exists_by_username(&self, username: &str) -> anyhow::Result<bool> {
        let users = self.users();

        Ok(users
            .count_documents(doc! { "username": username }, None)
            .await?
            != 0)
    }
}

impl UsersRepo {
    pub async fn create(&self, user: &User) -> anyhow::Result<()> {
        let users = self.users();

        users.insert_one(user, None).await?;

        Ok(())
    }
}

impl UsersRepo {
    pub async fn verify_email(&self, uid: ObjectId) -> anyhow::Result<()> {
        let users = self.users();

        users
            .update_one(
                doc! { "_id": uid },
                doc! { "$set": { "email_verified": true } },
                None,
            )
            .await?;

        Ok(())
    }

    pub async fn update_avatar(&self) -> anyhow::Result<ObjectId> {
        // implement this. it will upload the image to some storage bucket and set its id in User
        todo!()
    }

    pub async fn update_full_name(&self, id: &ObjectId, name: Option<&str>) -> anyhow::Result<()> {
        let users = self.users();

        users
            .update_one(doc! { "_id": id }, doc! { "$set": { "name": name } }, None)
            .await?;

        Ok(())
    }

    pub async fn update_bio(&self, id: &ObjectId, bio: Option<&str>) -> anyhow::Result<()> {
        let users = self.users();

        users
            .update_one(doc! { "_id": id }, doc! { "$set": { "bio": bio } }, None)
            .await?;

        Ok(())
    }

    pub async fn update_location(
        &self,
        id: &ObjectId,
        location: Option<&str>,
    ) -> anyhow::Result<()> {
        let users = self.users();

        users
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "location": location } },
                None,
            )
            .await?;

        Ok(())
    }

    pub async fn update_user(
        &self,
        id: &ObjectId,
        name: Option<&str>,
        bio: Option<&str>,
        location: Option<&str>,
    ) -> anyhow::Result<()> {
        let users = self.users();

        users
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "name": name, "bio": bio, "location": location } },
                None,
            )
            .await?;

        Ok(())
    }
}
