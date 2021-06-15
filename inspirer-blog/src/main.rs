#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate inspirer_actix_ext;

use std::io;
use inspirer_actix_ext::ModuleProvider;
use inspirer_actix_ext::config::config_provider;

mod server;
mod controller;
mod service;
mod model;
mod dao;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let mut module_provider = ModuleProvider::new();

    module_provider.register(config_provider(vec![])).await;

    info!("Start server.");
    server::start_server(module_provider).await
}
