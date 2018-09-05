#[macro_use] extern crate clap;
#[macro_use] extern crate log;

extern crate env_logger;
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate inspirer;
extern crate r2d2;

use std::path::PathBuf;
use std::sync::Arc;
use clap::{Arg, App as CommandApp};
use actix_web::{server, fs, App};
use inspirer::routes::blog::blog_routes;
use inspirer::state;
use inspirer::utils::helper;

fn main() {
    let matches = CommandApp::new("Inspirer")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Blog server.")
        .arg(
            Arg::with_name("signal")
                .short("s")
                .long("signal")
                .value_name("SIGNAL")
                .help("Send signal to server, support signal: start, stop, restart, reload")
                .takes_value(true)
        )
        .get_matches();

    match matches.value_of("signal").expect("You must provide signal option.") {
        "start" => start_server(),
        _ => (),
    };
}

fn start_server() {
    dotenv::dotenv();
    env_logger::init();

    let sys = actix::System::new("inspirer");

    let static_assets_handle = dotenv::var("STATIC_ASSETS_HANDLE").map(helper::convert_string_to_bool).unwrap_or(true);
    let static_assets_path = dotenv::var("STATIC_ASSETS_PATH").ok();
    let public_path = dotenv::var("PUBLIC_PATH").ok();

    let server_bind = dotenv::var("LISTEN").unwrap_or("127.0.0.1:8088".to_string());
    let state = state::AppState::new(state::Config {
        static_assets_handle,
        static_assets_path,
        public_path,
        database_url: dotenv::var("DATABASE_URL").expect("Error: DATABASE_URL is empty."),
        database_timezone: dotenv::var("DATABASE_TIMEZONE").ok(),
    });

    server::HttpServer::new(move || {
        let state = state.clone();
        let static_assets_path = Arc::clone(&state.static_assets_path);
        let public_path = Arc::clone(&state.public_path);
        let mut app = App::with_state(state.clone());

        debug!("Build app state.");

        if static_assets_handle {
            let path = match *static_assets_path {
                Some(ref static_assets_path) => PathBuf::from(static_assets_path).canonicalize().unwrap(),
                None => panic!("Error: You must provide static assets path when you set 'true' for STATIC_ASSETS_PATH")
            };

            let _ = match *public_path {
                Some(ref public_path) => PathBuf::from(public_path).canonicalize().unwrap(),
                None => panic!("Error: You must provide static assets path when you set 'true' for PUBLIC_PATH")
            };

            app = app.handler("/assets", fs::StaticFiles::new(path).unwrap());
        }

        app.scope("", blog_routes)
    }).bind(server_bind).unwrap().start();

    let _ = sys.run();
}