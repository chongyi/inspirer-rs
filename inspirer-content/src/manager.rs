use derive_builder::Builder;
use sea_orm::{Database, DatabaseConnection};

use crate::error::InspirerContentResult;

#[derive(Clone)]
pub struct Manager {
    pub(crate) database: DatabaseConnection,
}

#[derive(Debug, Builder, Default)]
pub struct ManagerConfig {
    /// 数据库地址
    database_url: String,
}

impl Manager {
    pub async fn create_from_config(config: ManagerConfig) -> InspirerContentResult<Self> {
        let database = Database::connect(&config.database_url).await?;
        tracing::info!("Created database component");

        tracing::info!("Created inspirer content manager module");
        Ok(Manager { database })
    }    
}
