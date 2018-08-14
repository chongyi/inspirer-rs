use actix::*;
use actix_web::*;
use diesel;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager };
use diesel::sql_types;

use error::Error;

pub struct DatabaseExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

pub type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;

no_arg_sql_function!(last_insert_id, sql_types::Unsigned<sql_types::BigInt>);

#[macro_export]
macro_rules! last_insert_id {
    ($conn:expr, $table:expr) => {
        {
            use $crate::diesel;
            use $crate::database::last_insert_id as lastid;
            use $crate::error::database::map_database_error as map_db_err;
            let generated_id: u64 = diesel::select(lastid)
                .first($conn)
                .map_err(map_db_err($table))?;

            generated_id
        }
    };
}

#[macro_export]
macro_rules! delete_by_id {
    ($conn:expr => ($table:ident # = $id:expr)) => {
        delete_by_id!($conn => ($table id = $id))
    };
    ($conn:expr => ($table:ident $id_field:ident = $id:expr)) => {
        {
            use $crate::error::database::map_database_error as map_db_err;
            diesel::delete($table)
                .filter($id_field.eq($id))
                .execute($conn)
                .map_err(map_db_err(Some(stringify!($table))))
        }
    };
}

#[macro_export]
macro_rules! find_by_id {
    ($conn:expr => ($table:ident # = $id:expr => $ty:ty)) => {
        find_by_id!($conn => ($table id = $id => $ty))
    };
    ($conn:expr => ($table:ident($fields:expr) # = $id:expr => $ty:ty)) => {
        find_by_id!($conn => ($table($fields) id = $id => $ty))
    };
    ($conn:expr => ($table:ident $id_field:ident = $id:expr => $ty:ty)) => {
        {
            use $crate::error::database::map_database_error as map_db_err;
            $table
                .filter($id_field.eq($id))
                .first::<$ty>($conn)
                .map_err(map_db_err(Some(stringify!($table))))
        }
    };
    ($conn:expr => ($table:ident($fields:expr) $id_field:ident = $id:expr => $ty:ty)) => {
        {
            use $crate::error::database::map_database_error as map_db_err;
            $table
                .select($fields)
                .filter($id_field.eq($id))
                .first::<$ty>($conn)
                .map_err(map_db_err(Some(stringify!($table))))
        }
    };
    ($conn:expr => ($table:ident($fields:expr) filter($filter:expr) => $ty:ty)) => {
        {
            use $crate::error::database::map_database_error as map_db_err;
            $table
                .select($fields)
                .filter($filter)
                .first::<$ty>($conn)
                .map_err(map_db_err(Some(stringify!($table))))
        }
    };
}

#[macro_export]
macro_rules! update_by_id {
    ($conn:expr => ($table:ident # = $id:expr; <- $update:expr)) => {
        update_by_id!($conn => ($table id = $id; <- $update))
    };
    ($conn:expr => ($table:ident $id_field:ident = $id:expr; <- $update:expr)) => {
        update_by_id!($conn => ($table filter($id_field.eq($id)); <- $update))
    };
    ($conn:expr => ($table:ident filter($filter:expr); <- $update:expr)) => {
        {
            use $crate::error::database::map_database_error as map_db_err;
            diesel::update($table)
                .set($update)
                .filter($filter)
                .execute($conn)
                .map_err(map_db_err(Some(stringify!($table))))
        }
    };
}

#[macro_export]
macro_rules! paginator {
    ($conn:ident, $w:ident, $rt:ty, $lg:block) => {
        {
            use $crate::message::PaginatedListMessage as PaginatedMessage;
            use $crate::error::database::map_database_error as map_db_err;
            use $crate::result::Result as UResult;
            let paginator = || -> UResult<PaginatedMessage<$rt>> {
                let counter = || { $lg };
                let getter = || { $lg };

                let count = counter().count().first::<i64>($conn).map_err(map_db_err(Some("<paginating>")))?;
                let results = getter()
                    .limit($w.per_page)
                    .offset(($w.page - 1) * $w.per_page)
                    .load::<$rt>($conn).map_err(map_db_err(Some("<paginating>")))?;

                Ok(PaginatedMessage { list: results, total: count, page: $w.page, per_page: $w.per_page })
            };

            paginator
        }
    };
    ($conn:ident, $fields:expr, $w:ident, $rt:ty, $lg:block) => {
        {
            use $crate::message::PaginatedListMessage as PaginatedMessage;
            use $crate::error::database::map_database_error as map_db_err;
            use $crate::result::Result as UResult;
            let paginator = || -> UResult<PaginatedMessage<$rt>> {
                let counter = || { $lg };
                let getter = || { $lg };

                let count = counter().count().first::<i64>($conn).map_err(map_db_err(Some("<paginating>")))?;
                let results = getter()
                    .select($fields)
                    .limit($w.per_page)
                    .offset(($w.page - 1) * $w.per_page)
                    .load::<$rt>($conn).map_err(map_db_err(Some("<paginating>")))?;

                Ok(PaginatedMessage { list: results, total: count, page: $w.page, per_page: $w.per_page })
            };

            paginator
        }
    };
}

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

impl DatabaseExecutor {
    pub fn connection(&mut self) -> Result<Conn, Error> {
        Ok(self.0.get().or(Err(Error))?)
    }
}