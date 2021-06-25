#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate inspirer_actix_ext;
#[macro_use]
extern crate strum;
#[macro_use]
extern crate validator;

use std::io;
use inspirer_actix_ext::ModuleProvider;
use inspirer_actix_ext::config::{config_provider, ConfigProvider};
use inspirer_actix_ext::database;
use clap::{App, Arg};

mod server;
mod controller;
mod service;
mod model;
mod dao;
mod error;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let matches = App::new("Inspirer Blog")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .takes_value(true))
        .get_matches();

    let config_file = matches.value_of("config")
        .map(|config_file_name| vec![ConfigProvider::String(config_file_name.into())])
        .unwrap_or(vec![]);

    let mut module_provider = ModuleProvider::new();

    module_provider.register(config_provider(config_file)).await;
    module_provider.register(database::mysql::register).await;

    info!("Start server.");
    server::start_server(module_provider).await
}
