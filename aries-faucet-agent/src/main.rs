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

use crate::config::get_app_env_config;
use crate::logging::*;
use crate::server::start_server;

mod logging;
mod application;
mod server;
mod config;
mod common;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_config = get_app_env_config();
    init_logger(app_config.log_level.clone(), app_config.log_format.clone());
    start_server(app_config.clone()).await
}
