use super::payload::UserCreatePayload;
use common::hashing;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    username: String,
    pub email: String,
    email_verified: bool,
    name: Option<String>,
    avatar_id: Option<Uuid>,
    bio: Option<String>,
    location: Option<String>,
    password: String,
    created_at: DateTime<Utc>,
}

impl User {
    pub fn get_password_hash(&self) -> &str {
        &self.password
    }
}

impl User {
    pub fn from_create_payload(payload: UserCreatePayload) -> anyhow::Result<Self> {
        let id = ObjectId::new();
        let password = hashing::hash_password(payload.password.as_bytes())?;
        let created_at = Utc::now();

        Ok(Self {
            _id: id,
            username: payload.username,
            email: payload.email,
            email_verified: false,
            name: payload.name,
            avatar_id: None,
            bio: None,
            location: None,
            password,
            created_at,
        })
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    username: String,
    email: String,
    email_verified: bool,
    name: Option<String>,
    avatar_id: Option<Uuid>,
    bio: Option<String>,
    location: Option<String>,
    created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value._id.to_string(),
            username: value.username,
            email: value.email,
            email_verified: value.email_verified,
            name: value.name,
            avatar_id: value.avatar_id,
            bio: value.bio,
            location: value.location,
            created_at: value.created_at,
        }
    }
}
