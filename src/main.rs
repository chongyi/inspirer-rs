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
extern crate mime;

#[macro_use]
mod util;
mod controllers;
mod database;
mod state;
mod models;
mod schema;
mod middlewares;

use actix::*;
use actix_web::*;
use actix_web::http::Method;
use actix_web::middleware::session::SessionStorage;
use util::auth::JWTSessionBackend;
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::sql_query;

use controllers::admin;
use middlewares::authenticate::Authenticate as MAuthenticate;

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
        let cloned = pool.clone();
        let connection = &cloned.get().expect("Error: Connection initialize error.");
        let timezone = dotenv::var("DB_TIMEZONE").unwrap_or("+8:00".to_string());

        sql_query(format!("set time_zone='{}'", timezone)).execute(connection).expect("Error: Connection initialize error.");;

        database::DatabaseExecutor(cloned)
    });

    server::HttpServer::new(
        move || App::with_state(state::AppState { database: addr.clone() })
            .middleware(SessionStorage::new(JWTSessionBackend))
            .scope("/api.admin", |scope| {
                scope
                    .route("/authentication", Method::POST, admin::authorization::authorization)
                    .nested("", |scope| {
                        scope.middleware(MAuthenticate)
                            .route("/session/current-user", Method::GET, admin::user::get_current_user_info)
                            .route("/category", Method::GET, admin::category::get_category_list)
                            .route("/category", Method::POST, admin::category::create_category)
                            .route("/category/{id:\\d+}", Method::DELETE, admin::category::delete_category)
                            .route("/category/{id:\\d+}", Method::PUT, admin::category::update_category)
                    })
            })
    ).bind(server_bind).unwrap().start();

    let _ = sys.run();
}