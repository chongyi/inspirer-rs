#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate actix_web;

pub(crate) mod handler;
pub mod middleware;
pub mod routes;
pub mod app;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
