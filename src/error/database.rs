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