use std::collections::HashMap;
use std::fmt;
use message::ErrorMessage;
use diesel::result::{
    Error as DieselError,
    DatabaseErrorKind as DieselDatabaseErrorKind,
};

use super::error_msg;
use super::Error;
use super::ErrorDetail;

pub const NOT_FOUND: u16 = 10022;
pub const CONFLICT: u16 = 10013;
pub const GET_CONNECTION_ERROR: u16 = 10001;
pub const UNKNOWN_DB_ERROR: u16 = 10099;

pub fn map_database_error(target: Option<&'static str>) -> impl FnOnce(DieselError) -> Error {
    move |err: DieselError| {
        let target = target.unwrap_or("[unknown]");
        match err {
            DieselError::NotFound => Error::not_found_error::<DieselError>(None, Some(error_msg(NOT_FOUND, "Resource or data not found.", Some(ErrorDetail::String(format!("Target table: {}", target)))))),
            DieselError::DatabaseError(kind, info) => {
                let mut detail = HashMap::new();
                detail.insert("message".to_string(), Some(ErrorDetail::String(info.message().to_string())));
                detail.insert("table".to_string(), info.table_name().map(|v| ErrorDetail::String(v.to_string())));
                detail.insert("detail".to_string(), info.details().map(|v| ErrorDetail::String(v.to_string())));

                match kind {
                    DieselDatabaseErrorKind::UniqueViolation => Error::conflict::<DieselError>(None, Some(error_msg(NOT_FOUND, "Data conflict.", Some(ErrorDetail::Hash(detail))))),
                    _ => Error::internal_server_error::<DieselError>(None, Some(error_msg(UNKNOWN_DB_ERROR, "Unknown database error.", Some(ErrorDetail::Hash(detail)))))
                }
            }
            _ => Error::internal_server_error::<DieselError>(None, Some(error_msg(UNKNOWN_DB_ERROR, "Unknown database error.", Some(ErrorDetail::String(format!("Table: {}, Error: {:?}", target, err))))))
        }
    }
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        (map_database_error(None))(err)
    }
}

impl Error {
    pub fn database_connection_get_error<T>(err: T) -> Error
        where T: fmt::Debug + Send + Sync + 'static
    {
        Error::internal_server_error::<DieselError>(None, Some(error_msg(
            GET_CONNECTION_ERROR,
            "Database connect error.",
            Some(ErrorDetail::String(format!("{:?}", err)))
        )))
    }
}