#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate inspirer_actix;

pub mod middleware;
pub mod routes;
pub mod app;
pub(crate) mod controller;

pub mod result {
    pub use inspirer_actix::response::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
