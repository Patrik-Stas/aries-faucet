use std::rc::Rc;

use crate::application::service::service_connections::ServiceConnections;
use crate::application::storage::connections::ConnectionResource;
use crate::application::storage::build_mongodb_client;
use crate::get_app_env_config;

pub mod storage;
mod service;
pub mod error;

pub struct Application {
    pub(crate) service_connections: Rc<ServiceConnections>,
}

pub async fn build_application() -> Result<Application, ()> {
    let config = get_app_env_config();
    log::info!("Building application using configuration {:?}", config);

    log::info!("Building Mongo client");
    let mongo_client = build_mongodb_client(&config.mongodb_uri).await.unwrap();
    let collection_connections = mongo_client
        .database("rvcxs")
        .collection(ConnectionResource::COLLECTION_NAME);
    let service_connections = Rc::new(ServiceConnections::new(collection_connections));
    Ok(Application {
        service_connections,
    })
}
