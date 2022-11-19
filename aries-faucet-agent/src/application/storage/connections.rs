use bson::oid::ObjectId;
use mongodb::Client;
use serde::{Deserialize, Serialize, Serializer};

impl Connection {
    pub const COLLECTION_NAME: &'static str = "connections";
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Connection {
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId,
    pub(crate) label: String,

}
