use actix::*;
use actix_web::*;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager };

pub struct DatabaseExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

impl DatabaseExecutor {
    pub fn connection(&mut self) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, Error> {
        Ok(self.0.get().map_err(error::ErrorInternalServerError)?)
    }
}

