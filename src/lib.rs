#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate tera;
#[macro_use] extern crate log;

extern crate serde;
extern crate serde_json;
extern crate actix;
extern crate actix_web;
extern crate mime;
extern crate chrono;
extern crate futures;
extern crate regex;
extern crate comrak;

#[macro_use] mod database;
mod models;
mod schema;
mod error;
mod message;
mod controllers;

mod template {
    use tera::Tera;
    lazy_static! {
        pub static ref TEMPLATES: Tera = {
            let mut tera = compile_templates!("res/templates/**/*");
            tera.autoescape_on(vec!["html", ".sql"]);
            tera
        };
    }
}

pub mod state;
pub mod routes;

pub mod result {
    use std::result::Result as StdResult;
    use error::Error;

    pub type Result<T> = StdResult<T, Error>;
}