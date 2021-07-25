#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate strum;

pub mod model;
pub mod dao;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
