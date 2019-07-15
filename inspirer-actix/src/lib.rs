#[macro_use]
extern crate log;

pub mod error;
#[macro_use]
pub mod macros;
pub mod response;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
