#[macro_use] extern crate clap;
#[macro_use] extern crate log;

extern crate env_logger;
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate inspirer;
extern crate r2d2;

use clap::{Arg, App as CommandApp, SubCommand, ArgMatches};
use actix::SyncArbiter;
use actix_web::{server, fs, App};
use inspirer::routes::blog::blog_routes;
use inspirer::state;

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

    let server_bind = dotenv::var("LISTEN").unwrap_or("127.0.0.1:8088".to_string());
    let state = state::AppState::new(state::Config {
        database_url: dotenv::var("DATABASE_URL").expect("Error: DATABASE_URL is empty."),
        database_timezone: dotenv::var("DB_TIMEZONE").unwrap_or("+8:00".to_string())
    });

    server::HttpServer::new(
        move || App::with_state(state.clone())
            .handler("/assets", fs::StaticFiles::new("./res/public/assets").unwrap())
            .scope("", blog_routes)
    ).bind(server_bind).unwrap().start();

    let _ = sys.run();
}