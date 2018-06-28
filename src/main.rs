#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate failure;

extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate actix;
extern crate actix_web;
extern crate actix_web_httpauth;
extern crate dotenv;
extern crate chrono;
extern crate http;
extern crate djangohashers;
extern crate biscuit;

mod controllers;
mod database;
mod state;
mod models;
mod schema;
mod util;
mod middlewares;

use actix::*;
use actix_web::*;
use actix_web::http::Method;
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};

use controllers::admin;

fn main() {
    start_server();
}

fn start_server() {
    let sys = actix::System::new("inspirer");

    let database_url = dotenv::var("DATABASE_URL").expect("Error: DATABASE_URL is empty.");
    let server_bind = dotenv::var("LISTEN").unwrap_or("127.0.0.1:8088".to_string());

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Error: Failed to build pool");
    let addr = SyncArbiter::start(8, move || {
        database::DatabaseExecutor(pool.clone())
    });

    server::HttpServer::new(
        move || App::with_state(state::AppState { database: addr.clone() })
            .scope("/api.admin", |scope| {
                scope
                    .route("/authentication", Method::POST, admin::authorization::authorization)
            })
    ).bind(server_bind).unwrap().start();

    let _ = sys.run();
}