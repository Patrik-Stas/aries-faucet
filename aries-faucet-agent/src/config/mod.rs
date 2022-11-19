use std::io;
use std::io::prelude::*;
use std::fs::{File, read_to_string};
use envconfig::Envconfig;
use lazy_static::lazy_static;
use dotenv::dotenv;
use std::env;

fn try_load_env_config() -> () {
    dotenv().ok();
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}

lazy_static! {
    static ref APP_ENV_CONFIG: AppEnvConfig = {
        try_load_env_config();
        AppEnvConfig::init().unwrap()
    };
}

pub fn get_app_env_config() -> &'static AppEnvConfig {
    return &APP_ENV_CONFIG
}

#[derive(Envconfig, Debug, Clone)]
pub struct AppEnvConfig {
    #[envconfig(from = "HOST")]
    pub host: String,
    #[envconfig(from = "PORT")]
    pub port: u16,
    #[envconfig(from = "MONGODB_URI")]
    pub mongodb_uri: String,
    #[envconfig(from = "ENABLE_TLS")]
    pub enable_tls: bool,
    #[envconfig(from = "BINDING_ADDRESS")]
    pub binding_address: String,
    #[envconfig(from = "SERVER_WORKERS", default = "4")]
    pub server_workers: usize,
    #[envconfig(from = "GRACEFUL_SHUTDOWN_TIMEOUT_SEC", default = "20")]
    pub server_graceful_shutdown_timeout_sec: u64,
    #[envconfig(from = "CORS_PERMISSIVE", default = "false")]
    pub cors_permissive: bool,
    #[envconfig(from = "CORS_ALLOW_ORIGIN")]
    pub cors_allow_origin: String,
    #[envconfig(from = "LOG_LEVEL")]
    pub log_level: String,
    #[envconfig(from = "LOG_FORMAT")]
    pub log_format: String,

}
