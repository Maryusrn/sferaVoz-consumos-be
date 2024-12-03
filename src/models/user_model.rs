use serde::{Serialize, Deserialize};
use mongodb::bson::{doc, oid::ObjectId};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String,
}
