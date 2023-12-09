use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    _id: ObjectId,
}

impl RefreshToken {
    pub fn new(id: ObjectId) -> Self {
        Self { _id: id }
    }
}
