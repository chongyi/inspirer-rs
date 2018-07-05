use diesel::result::{
    Error as DieselError,
    DatabaseErrorKind as DieselDatabaseErrorKind,
    DatabaseErrorInformation
};

use actix_web::http::StatusCode;

use super::{ApplicationError, ErrorInformation};

#[macro_export]
macro_rules! map_database_error {
    ($target:expr) => {
        |err| {
            use util::error::ApplicationError as IAppErr;
            use diesel::result::{
                Error as IDieselError,
                DatabaseErrorKind as IDieselDatabaseErrorKind,
                DatabaseErrorInformation as IDatabaseErrorInformation
            };
            let t = $target.into();
            match err {
                IDieselError::NotFound => IAppErr::DbNotFound(t),
                IDieselError::DatabaseError(kind, info) => match kind {
                    IDieselDatabaseErrorKind::UniqueViolation => IAppErr::DbConflict(t, kind),
                    _ => IAppErr::DbError(t, kind),
                },
                _ => IAppErr::DbUnknownError(),
            }
        }
    };
}

pub enum DatabaseErrorKind {
    NotFound(String),
    DatabaseError(String, DieselDatabaseErrorKind),
    GetConnection,
    PaginationError,
    Unknown,
}

impl ApplicationError {
    pub const DB_NOT_FOUND: (u16, &'static str, StatusCode) = (10441, "Data not found.", StatusCode::NOT_FOUND);
    pub const DB_UNIQUE_VIOLATION: (u16, &'static str, StatusCode) = (10412, "Data conflict.", StatusCode::CONFLICT);
    pub const DB_ERR: (u16, &'static str, StatusCode) = (10400, "Database error.", StatusCode::INTERNAL_SERVER_ERROR);
    pub const DB_GET_CONNECTION: (u16, &'static str, StatusCode) = (10402, "Database connection.", StatusCode::INTERNAL_SERVER_ERROR);
    pub const DB_PAGINATION_ERROR: (u16, &'static str, StatusCode) = (10444, "Format data error.", StatusCode::INTERNAL_SERVER_ERROR);

    #[allow(non_snake_case)]
    pub fn DbNotFound(target: String) -> Self {
        let (a, b, c) = Self::DB_NOT_FOUND;
        ApplicationError::DatabaseError(DatabaseErrorKind::NotFound(target), ErrorInformation::new(
            a, b.into(), c, None
        ))
    }

    #[allow(non_snake_case)]
    pub fn DbConflict(target: String, kind: DieselDatabaseErrorKind) -> Self {
        let (a, b, c) = Self::DB_UNIQUE_VIOLATION;
        ApplicationError::DatabaseError(DatabaseErrorKind::DatabaseError(target, kind), ErrorInformation::new(
            a, b.into(), c, None
        ))
    }

    #[allow(non_snake_case)]
    pub fn DbError(target: String, kind: DieselDatabaseErrorKind) -> Self {
        let (a, b, c) = Self::DB_ERR;
        ApplicationError::DatabaseError(DatabaseErrorKind::DatabaseError(target, kind), ErrorInformation::new(
            a, b.into(), c, None
        ))
    }

    #[allow(non_snake_case)]
    pub fn DbUnknownError() -> Self {
        let (a, b, c) = Self::DB_ERR;
        ApplicationError::DatabaseError(DatabaseErrorKind::Unknown, ErrorInformation::new(
            a, b.into(), c, None
        ))
    }

    #[allow(non_snake_case)]
    pub fn DbGetConnectionError() -> Self {
        let (a, b, c) = Self::DB_GET_CONNECTION;
        ApplicationError::DatabaseError(DatabaseErrorKind::GetConnection, ErrorInformation::new(
            a, b.into(), c, None
        ))
    }

    #[allow(non_snake_case)]
    pub fn DbPaginationError() -> Self {
        let (a, b, c) = Self::DB_PAGINATION_ERROR;
        ApplicationError::DatabaseError(DatabaseErrorKind::PaginationError, ErrorInformation::new(
            a, b.into(), c, None
        ))
    }
}

pub fn map_database_error(target: &'static str) -> Box<FnOnce(DieselError) -> ApplicationError> {
    Box::new(move |err| {
        let t = target.into();
        match err {
            DieselError::NotFound => ApplicationError::DbNotFound(t),
            DieselError::DatabaseError(kind, info) => match kind {
                DieselDatabaseErrorKind::UniqueViolation => ApplicationError::DbConflict(t, kind),
                _ => ApplicationError::DbError(t, kind),
            },
            _ => ApplicationError::DbUnknownError(),
        }
    })
}