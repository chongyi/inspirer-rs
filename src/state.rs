use actix::*;

use database::DatabaseExecutor;

pub struct AppState {
    /// 通过该字段对数据库进行访问以及操作
    pub database: Addr<DatabaseExecutor>,
}