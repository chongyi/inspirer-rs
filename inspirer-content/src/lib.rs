mod entity;
pub mod manager;
pub mod error;
mod dao;
pub mod model;
pub mod service;
pub mod enumerate;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
