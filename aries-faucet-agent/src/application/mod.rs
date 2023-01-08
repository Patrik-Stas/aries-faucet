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
    let agent = initialize(&app_config).await;

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

pub async fn initialize(app_config: &AppEnvConfig) -> AriesAgent {
    let agent_id = uuid::Uuid::new_v4();
    let enterprise_seed= "000000000000000000000000Trustee1".to_string();
    let wallet_config = WalletInitConfig {
        wallet_name: format!("rust_agent_{}", agent_id),
        wallet_key: "8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY".to_string(),
        wallet_kdf: "RAW".to_string(),
    };
    let pool_config = PoolInitConfig {
        genesis_path: app_config.genesis_path.clone(),
        pool_name: format!("pool_{}", agent_id),
    };
    let init_config = InitConfig {
        enterprise_seed,
        pool_config,
        agency_config: None,
        wallet_config,
        service_endpoint: "http://localhost:8080".into(),
    };
    AriesAgent::initialize(init_config).await.unwrap()
}