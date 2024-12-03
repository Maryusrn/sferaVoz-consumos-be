use serde::{Serialize, Deserialize};
use mongodb::bson::{doc, oid::ObjectId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Calls {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub entrada: String,
    pub salida: String,
    pub fecha: String,
    pub hora: String,
    pub duracion: String,
}
