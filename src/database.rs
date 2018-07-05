use actix::*;
use actix_web::*;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager };
use diesel::sql_types;

use util::error::ApplicationError as Error;

pub struct DatabaseExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

pub type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;

no_arg_sql_function!(last_insert_id, sql_types::Unsigned<sql_types::BigInt>);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

impl DatabaseExecutor {
    pub fn connection(&mut self) -> Result<Conn, Error> {
        Ok(self.0.get().or(Err(Error::DbGetConnectionError()))?)
    }
}