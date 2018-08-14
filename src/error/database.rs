use diesel::result::{
    Error as DieselError,
    DatabaseErrorKind as DieselDatabaseErrorKind,
};

use super::Error;

pub fn map_database_error(target: Option<&str>) -> impl FnOnce(DieselError) -> Error {
    |err| {
        let err = Error;
        err
    }
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        (map_database_error(None))(err)
    }
}