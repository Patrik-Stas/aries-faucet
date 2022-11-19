use bson::oid::ObjectId;
use mongodb::Client;
use serde::{Serializer};

pub mod db_util;
pub mod connections;
pub mod filters;

pub async fn build_mongodb_client(mongodb_uri: &str) -> Result<Client, anyhow::Error> {
    let client = Client::with_uri_str(mongodb_uri).await
        .map_err(|err| anyhow!(format!("Failed to build Mongo client, err: {}", err)))?;
    Ok(client)
}

pub fn serialize_object_id<S>(oid: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    match oid.as_ref().map(|x| x.to_hex()) {
        Some(v) => s.serialize_str(&v),
        None => s.serialize_none()
    }
}
