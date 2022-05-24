mod server;
mod controller;
mod route;
mod response;
mod error;
mod request;

fn main() {
    dotenv::dotenv().expect("Initialize dotenv error.");
    
    server::run().unwrap();
}