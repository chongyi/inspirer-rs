use std::rc::Rc;
use actix::*;
use diesel::{sql_query, RunQueryDsl};
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use database::DatabaseExecutor;

pub struct Config {
    pub database_url: String,
    pub database_timezone: String,
}

#[derive(Clone)]
pub struct AppState {
    /// 通过该字段对数据库进行访问以及操作
    database: Addr<DatabaseExecutor>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let timezone = config.database_timezone;
        let manager = ConnectionManager::<MysqlConnection>::new(config.database_url);
        let pool = Pool::builder().build(manager).expect("Error: Failed to build pool");

        let addr = SyncArbiter::start(8, move || {
            let cloned = pool.clone();
            let connection = &cloned.get().expect("Error: Connection initialize error.");

            sql_query(format!("set time_zone='{}'", timezone.clone()))
                .execute(connection)
                .expect("Error: Connection initialize error.");;

            DatabaseExecutor(cloned)
        });

        AppState {
            database: addr
        }
    }
}