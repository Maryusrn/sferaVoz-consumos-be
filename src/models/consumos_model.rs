use serde::{Serialize, Deserialize};
use mongodb::bson::{doc, oid::ObjectId};
use mongodm::bson::DateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumos {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub fecha_ini: DateTime,
    pub fecha_fin: DateTime,
}