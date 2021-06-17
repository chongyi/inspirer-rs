use std::io;

use actix_web::{App, HttpServer, web};

use crate::controller;
use inspirer_actix_ext::{ModuleContainer, ModuleProvider};
use inspirer_actix_ext::config::Config;
use actix_web::middleware::Logger;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub listen: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            listen: "0.0.0.0:8006".into(),
        }
    }
}

pub async fn start_server(module_provider: ModuleProvider) -> io::Result<()> {
    let config = module_provider.get::<Config>().expect("Cannot find configuration object.");
    let server_config = config.get::<ServerConfig>("server").unwrap_or_default();
    let module_container = module_provider.into_module_container();

    HttpServer::new(move || {
        App::new()
            .configure(module_container.clone().module_provider())
            .wrap(Logger::default())
            .service(controller::index::home)
            .service(controller::index::item)
    })
        .bind(server_config.listen.as_str())?
        .run()
        .await
}