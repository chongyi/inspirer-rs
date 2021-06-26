use std::io;

use actix_web::{App, HttpServer, web, HttpMessage};
use actix_web::middleware::Logger;
use inspirer_actix_ext::{ModuleContainer, ModuleProvider};
use inspirer_actix_ext::config::Config;
use inspirer_actix_ext::validator::ValidateConfig;

use crate::config::ServerConfig;
use crate::controller;
use crate::error::{Error, RuntimeError};
use actix_web::web::{scope, Data};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::service::auth::AuthTokenService;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;

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
            .service(controller::index::get_content)
            .service(controller::index::get_content_list)
            .service(controller::auth::login)
            .service(scope("")
                .wrap(HttpAuthentication::bearer(token_validator))
                .service(controller::auth::status)
                .service(controller::admin::create_content)
                .service(controller::admin::update_content)
                .service(controller::admin::delete_content)
            )
    })
        .bind(server_config.listen.as_str())?
        .run()
        .await
}

async fn token_validator(req: ServiceRequest, bearer: BearerAuth) -> Result<ServiceRequest, actix_web::Error> {
    let config = req.app_data::<Data<Config>>().ok_or(Error::RuntimeError(RuntimeError::UnknownError))?;
    let service = AuthTokenService::new(
        config.get_ref().clone()
    );

    let payload = service.extract(bearer.token()).map_err(Error::Anyhow)?;

    req.extensions_mut().insert(payload);
    Ok(req)
}