use std::fmt::Display;

use serde::Serialize;

pub mod content;

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub msg: String,
}

pub fn error_message_from_err<E: Display>(err: &E) -> ErrorMessage {
    ErrorMessage {
        msg: format!("{}", err)
    }
}