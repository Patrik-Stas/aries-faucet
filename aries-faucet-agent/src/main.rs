#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate bson;

extern crate envconfig;
#[macro_use]
extern crate envconfig_derive;
extern crate dotenv;

extern crate lazy_static;
extern crate tokio;

use crate::config::{AppEnvConfig, get_app_env_config};
use crate::logging::*;
use crate::server::start_server;

mod logging;
mod application;
mod server;
mod config;
mod common;

use aries_vcx_agent::{Agent as AriesAgent, InitConfig, PoolInitConfig, WalletInitConfig};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_config = get_app_env_config();
    init_logger(app_config.log_level.clone(), app_config.log_format.clone()).unwrap();
    start_server(app_config.clone()).await
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