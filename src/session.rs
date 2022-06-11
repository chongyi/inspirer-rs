use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Claims {
    pub sub: u128,
    pub exp: usize,
    pub iat: usize,
}