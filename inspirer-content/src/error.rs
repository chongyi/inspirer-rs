pub type InspirerContentResult<T, E = Error> = Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {

}