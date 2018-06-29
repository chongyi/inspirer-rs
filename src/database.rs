use actix::*;
use actix_web::*;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager };
use diesel::types;

pub struct DatabaseExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

no_arg_sql_function!(last_insert_id, types::Unsigned<types::BigInt>);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

impl DatabaseExecutor {
    pub fn connection(&mut self) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, Error> {
        Ok(self.0.get().map_err(error::ErrorInternalServerError)?)
    }
}

pub type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;