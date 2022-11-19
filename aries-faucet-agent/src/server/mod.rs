use std::{fs::File, io::BufReader};
use std::time::Duration;

use actix_web::{App, HttpServer, middleware, web};
use actix_web::http::{header, KeepAlive};
use actix_web::middleware::Logger;
use log::debug;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use actix_cors::Cors;
use actix_web::dev::AppConfig;

use crate::application::build_application;
use crate::config::AppEnvConfig;
use crate::get_app_env_config;
use crate::server::api::{api_connections, api_health};

mod api;
pub mod response;


fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file = &mut BufReader::new(File::open("certs/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("certs/key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

pub async fn start_server(app_config: AppEnvConfig) -> std::io::Result<()> {
    let app_config_cl = app_config.clone();
    let mut server = HttpServer::new(move ||
        {
            let middleware_cors = match app_config_cl.cors_permissive {
                true => Cors::permissive(),
                false => Cors::default()
                    .allowed_origin(&app_config_cl.cors_allow_origin)
                    .allowed_methods(vec!["GET", "POST"])
                    .supports_credentials()
                    .max_age(3600),
            };
            let middleware_logger = Logger::default();

            App::new()
                .wrap(middleware_logger)
                .wrap(middleware_cors)
                .wrap(middleware::Compress::default())
                .data_factory(move || build_application())
                .service(
                    web::scope("/api/health")
                        .route("", web::get().to(api_health::get_health))
                )
                .service(
                    web::scope("/api/connections")
                        .route("", web::get().to(api_connections::get_many))
                        .route("", web::post().to(api_connections::create))
                        .route("{id}", web::get().to(api_connections::get_by_id))
                        .route("{id}", web::put().to(api_connections::update_by_id))
                        .route("{id}", web::delete().to(api_connections::delete_by_id))
                )
        });


    let socket_address = format!("{}:{}", app_config.binding_address, app_config.port);

    if app_config.enable_tls {
        let tls_config = load_rustls_config();
        server = server.bind_rustls(&socket_address, tls_config)?
    } else {
        server = server.bind(&socket_address)?
    }

    server
        .keep_alive(KeepAlive::default() )
        .shutdown_timeout(app_config.server_graceful_shutdown_timeout_sec)
        .workers(app_config.server_workers)
        .run()
        .await
}
