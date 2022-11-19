use anyhow::anyhow;
use bson::{Document, oid::ObjectId};
use futures::{StreamExt, TryStreamExt};
use crate::application::error::{BusinessError, BusinessResult};

use crate::application::storage::connections::Connection;
use crate::application::storage::db_util::{CursorIntoVec, struct_into_document};

pub struct ServiceConnections {
    collection_connections: mongodb::Collection<Connection>,
}

impl ServiceConnections {
    pub fn new(collection_resources: mongodb::Collection<Connection>) -> ServiceConnections {
        ServiceConnections { collection_connections: collection_resources }
    }

    pub async fn create(&self, label: String) -> BusinessResult<String> {
        log::info!("db_create_resource >> label: {}", label);
        let connection = Connection {
            id: ObjectId::new(),
            label,
        };
        let rs = self.collection_connections.insert_one(connection, None).await
            .map_err(|e| anyhow!("service_connections::create >> failed create document, root cause: {}", e.to_string()))?;
        let inserted_id: String = rs
            .inserted_id
            .as_object_id()
            .map(ObjectId::to_hex)
            .ok_or_else(|| anyhow!("[MongodbCrudService::save] Failed to get inserted id"))?;
        Ok(inserted_id)
    }

    pub(crate) async fn get_many(&self, filter: Document) -> BusinessResult<Vec<Connection>> {
        let mut cursor = self.collection_connections.find(Some(filter), None).await
            .map_err(|e| anyhow!("service_connections::get_many >> failed document find, root cause: {}", e.to_string()))?;
        let mut connections = vec![];
        while let Some(item) = cursor.next().await {
            let item = item.unwrap();
            connections.push(item)
        }
        return Ok(connections);
    }

    pub(crate) async fn get_by_id(&self, id: &str) -> BusinessResult<Connection> {
        let result = self.collection_connections
            .find_one(Some(doc! {"_id": ObjectId::parse_str(&id).unwrap() }), None).await
            .map_err(|e| anyhow!("service_connections::get_by_id >> failed document find, root cause: {}", e.to_string()))?
            .ok_or(BusinessError::ResourceNotFound(id.into()));
        log::info!("get_by_id >> result: {:?}", result);
        result
    }

    pub(crate) async fn delete_by_id(&self, id: &str) -> anyhow::Result<bool> {
        let result = self.collection_connections
            .delete_one(doc! {"_id": ObjectId::parse_str(&id).unwrap() }, None).await
            .map_err(|e| anyhow!("service_connections::delete_by_id >> failed document delete, root cause: {}", e.to_string()))?;
        match result.deleted_count {
            1 => Ok(true),
            0 => Ok(false),
            n @ _ => panic!("Tried to delete 1 document, but {} was deleted", n)
        }
    }

    pub(crate) async fn update_by_id(&self, id: &str, label: &str) -> anyhow::Result<bool> {
        let update = doc! {"$set": { "label": label } };
        let result = self.collection_connections
            .update_one( doc! {"_id": id}, update, None).await
            .map_err(|e| anyhow!("service_connections::update_by_id >> failed document update, root cause: {}", e.to_string()))?;
        match result.modified_count {
            1 => Ok(true),
            0 => Ok(false),
            n @ _ => panic!("Tried to modify 1 document, but {} was modified", n)
        }
    }
}

#[cfg(test)]
mod tests {
    use bson::oid::ObjectId;
    use log::info;
    use mongodb::Database;
    use uuid::Uuid;

    use crate::application::service::service_connections::ServiceConnections;
    use crate::application::storage::connections::Connection;
    use crate::application::storage::build_mongodb_client;

    struct IntegrationTest {
        service_connections: ServiceConnections,
        mongo_database: Database,
    }

    impl IntegrationTest {
        async fn clean_up(&self) {
            self.mongo_database.drop(None).await;
        }
    }

    async fn _create_service() -> IntegrationTest {
        let mongo_client = build_mongodb_client("mongodb://localhost:27017").await.unwrap();
        let database_name = Uuid::new_v4().to_string();
        let mongo_database = mongo_client.database(&database_name);
        let collection_connections = mongo_database.collection(Connection::COLLECTION_NAME);
        IntegrationTest {
            mongo_database,
            service_connections: ServiceConnections::new(collection_connections),
        }
    }

    #[tokio::test]
    async fn create_and_get() {
        let test = _create_service().await;
        let id = test.service_connections.create(
            "Alice".to_string(),
        ).await.unwrap();
        let connection = test.service_connections.get_by_id(&id).await.unwrap();
        assert!(connection.is_some());
        test.clean_up().await;
    }

    #[tokio::test]
    async fn create_and_get_many() {
        let test = _create_service().await;
        let id = test.service_connections.create(
            "Alice".to_string(),
        ).await.unwrap();
        let id = test.service_connections.create(
            "Faber".to_string(),
        ).await.unwrap();
        let connections = test.service_connections.get_many(doc! {}).await.unwrap();
        assert_eq!(connections.len(), 2);
        test.clean_up().await;
    }
}
