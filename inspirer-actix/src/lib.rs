#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

pub mod error;
#[macro_use]
pub mod macros;
pub mod response;
pub mod middleware;
pub mod auth;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
