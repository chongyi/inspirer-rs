use std::io;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use inspirer_actix_ext::{ModuleContainer, ModuleProvider};
use inspirer_actix_ext::config::Config;
use inspirer_actix_ext::validator::ValidateConfig;

use crate::config::ServerConfig;
use crate::controller;
use crate::error::Error;

pub async fn start_server(module_provider: ModuleProvider) -> io::Result<()> {
    let config = module_provider.get::<Config>().expect("Cannot find configuration object.");
    let server_config = config.get::<ServerConfig>("server").unwrap_or_default();
    let module_container = module_provider.into_module_container();

    HttpServer::new(move || {
        App::new()
            .configure(module_container.clone().module_provider())
            .app_data(ValidateConfig::default().error_handler(|err, _| Error::from(err).into()))
            .wrap(Logger::default())
            .service(controller::index::home)
            .service(controller::index::item)
    })
        .bind(server_config.listen.as_str())?
        .run()
        .await
}