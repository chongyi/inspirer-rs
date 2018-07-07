use actix::*;
use actix_web::*;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager };
use diesel::sql_types;

use util::error::ApplicationError as Error;

pub struct DatabaseExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

pub type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;

no_arg_sql_function!(last_insert_id, sql_types::Unsigned<sql_types::BigInt>);

#[macro_export]
macro_rules! last_insert_id {
    ($conn:expr, $table:expr) => {
        {
            use database::last_insert_id as lastid;
            use util::error::database::map_database_error as map_db_err;
            let generated_id: u64 = diesel::select(lastid)
                .first($conn)
                .map_err(map_db_err($table))?;

            generated_id
        }
    };
}

#[macro_export]
macro_rules! delete_by_id {
    ($table:ident, $table_name:expr, $conn:expr, $id:expr) => {
        delete_by_id!($table:ident, $table_name:expr, $conn:expr, $id:expr, id)
    };
    ($table:ident, $table_name:expr, $conn:expr, $id:expr, $id_field:ident) => {
        {
            use util::error::database::map_database_error as map_db_err;
            diesel::delete($table)
                .filter($id_field.eq($id))
                .execute($conn)
                .map_err(map_db_err($table_name))
        }
    };
}

#[macro_export]
macro_rules! find_by_id {
    ($ty:ty, $table:ident, $table_name:expr, $conn:expr, $id:expr) => {
        find_by_id!($ty, $table, $table_name, $conn, $id, id)
    };
    ($ty:ty, $table:ident, $table_name:expr, $conn:expr, $fields:expr, $id:expr) => {
        find_by_id!($ty, $table, $table_name, $conn, $fields, $id, id)
    };
    ($ty:ty, $table:ident, $table_name:expr, $conn:expr, $id:expr, $id_field:ident) => {
        {
            use util::error::database::map_database_error as map_db_err;
            $table
                .filter($id_field.eq($id))
                .first::<$ty>($conn)
                .map_err(map_db_err($table_name))
        }
    };
    ($ty:ty, $table:ident, $table_name:expr, $conn:expr, $fields:expr, $id:expr, $id_field:ident) => {
        {
            use util::error::database::map_database_error as map_db_err;
            $table
                .select($fields)
                .filter($id_field.eq($id))
                .first::<$ty>($conn)
                .map_err(map_db_err($table_name))
        }
    };
}

#[macro_export]
macro_rules! update_by_id {
    ($table:ident, $table_name:expr, $conn:expr, $id:expr, $update:expr) => {
        update_by_id!($table, $table_name, $conn, $id, id, $update)
    };
    ($table:ident, $table_name:expr, $conn:expr, $id:expr, $id_field:ident, $update:expr) => {
        {
            use util::error::database::map_database_error as map_db_err;
            diesel::update($table)
                .set($update)
                .filter($id_field.eq($id))
                .execute($conn)
                .map_err(map_db_err($table_name))
        }
    };
}

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

impl DatabaseExecutor {
    pub fn connection(&mut self) -> Result<Conn, Error> {
        Ok(self.0.get().or(Err(Error::DbGetConnectionError()))?)
    }
}