use diesel::result::{
    Error as DieselError,
    DatabaseErrorKind as DieselDatabaseErrorKind,
};

use actix_web::http::StatusCode;

use super::{ApplicationError, ErrorInformation};

pub fn map_database_error(table: &'static str) -> impl FnOnce(DieselError) -> ApplicationError {
    move |err| {
        let t = table.into();
        match err {
            DieselError::NotFound => ApplicationError::DbNotFound(t),
            DieselError::DatabaseError(kind, info) => match kind {
                DieselDatabaseErrorKind::UniqueViolation => ApplicationError::DbConflict(t, kind),
                _ => ApplicationError::DbError(t, kind),
            },
            _ => ApplicationError::DbUnknownError(),
        }
    }
}

pub enum DatabaseErrorKind {
    NotFound(String),
    DatabaseError(String, DieselDatabaseErrorKind),
    GetConnection,
    PaginationError,
    Unknown,
}

impl ApplicationError {
    error_trigger_define!(DB_NOT_FOUND, 10441, "Data not found.", StatusCode::NOT_FOUND);
    error_trigger_define!(DB_UNIQUE_VIOLATION, 10412, "Data conflict.", StatusCode::CONFLICT);
    error_trigger_define!(ApplicationError::DatabaseError, DB_ERR, 10400, "Database error.", StatusCode::INTERNAL_SERVER_ERROR, DatabaseErrorKind::Unknown, DbUnknownError);
    error_trigger_define!(ApplicationError::DatabaseError, DB_GET_CONNECTION, 10402, "Database connection.", StatusCode::INTERNAL_SERVER_ERROR, DatabaseErrorKind::GetConnection, DbGetConnectionError);
    error_trigger_define!(ApplicationError::DatabaseError, DB_PAGINATION_ERROR, 10444, "Format data error.", StatusCode::INTERNAL_SERVER_ERROR, DatabaseErrorKind::PaginationError, DbPaginationError);


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
}