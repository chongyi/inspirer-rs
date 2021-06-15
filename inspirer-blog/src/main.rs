#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod server;
mod controller;
mod service;
mod model;
mod dao;


fn main() {
    println!("Hello, world!");
}
