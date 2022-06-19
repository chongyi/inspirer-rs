use std::fmt::Display;

use inspirer_content::util::uuid::{uuid_to_base62, Uuid};
use serde::Serialize;

pub mod auth;
pub mod content;

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub msg: String,
}

pub fn error_message_from_err<E: Display>(err: &E) -> ErrorMessage {
    ErrorMessage {
        msg: format!("{}", err),
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct CreatedDataStringId {
    id: String,
}

impl CreatedDataStringId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        CreatedDataStringId {
            id: uuid_to_base62(uuid),
        }
    }
}
