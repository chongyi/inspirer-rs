use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::{BTreeMap};
use actix::*;
use actix::dev::{Request, ToEnvelope};
use diesel::{sql_query, RunQueryDsl};
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use database::DatabaseExecutor;

use models::category::CategoryBase;

pub struct Config {
    pub static_assets_handle: bool,
    pub static_assets_path: Option<String>,
    pub public_path: Option<String>,
    pub database_url: String,
    pub database_timezone: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub static_assets_handle: bool,
    pub static_assets_path: Arc<Option<String>>,
    pub public_path: Arc<Option<String>>,
    /// 通过该字段对数据库进行访问以及操作
    pub database: Addr<DatabaseExecutor>,
    /// 内置缓存
    pub built_in_cache: Arc<BuiltInCache>
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let timezone = config.database_timezone;
        let manager = ConnectionManager::<MysqlConnection>::new(config.database_url);
        let pool = Pool::builder().build(manager).expect("Error: Failed to build pool");

        let addr = SyncArbiter::start(8, move || {
            let cloned = pool.clone();
            let connection = &cloned.get().expect("Error: Connection initialize error.");
            let timezone = timezone.clone();

            if let Some(timezone) = timezone {
                sql_query(format!("set time_zone='{}'", timezone))
                    .execute(connection)
                    .expect("Error: Connection initialize error.");
            }

            DatabaseExecutor(cloned)
        });

        let static_assets_path = Arc::new(config.static_assets_path.clone());
        let public_path = Arc::new(config.public_path.clone());

        AppState {
            static_assets_handle: config.static_assets_handle,
            static_assets_path,
            public_path,
            database: addr,
            built_in_cache: Arc::new(BuiltInCache::default()),
        }
    }
}

#[derive(Clone)]
pub struct BuiltInCache {
    content_category: Arc<Mutex<BTreeMap<u32, CategoryBase>>>
}

impl Default for BuiltInCache {
    fn default() -> Self {
        BuiltInCache {
            content_category: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
}

impl BuiltInCache {
    pub fn get_category_by_id(&self, target: u32) -> Option<CategoryBase> {
        let resource = self.content_category.lock().unwrap();
        resource.get(&target).map(Clone::clone)
    }

    pub fn update_category(&self, source: CategoryBase) {
        let mut resource = self.content_category.lock().unwrap();
        let id = source.id;
        resource.insert(id, source);
    }
}