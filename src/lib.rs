#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;

extern crate serde;
extern crate serde_json;
extern crate actix;
extern crate actix_web;
extern crate mime;
extern crate chrono;

#[macro_use] mod database;
mod models;
mod schema;
mod error;
mod state;
mod message;

pub mod result {
    use std::result::Result as StdResult;
    use error::Error;

    pub type Result<T> = StdResult<T, Error>;
}