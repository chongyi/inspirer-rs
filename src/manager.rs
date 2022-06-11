use anyhow::Result;
use inspirer_content::manager::{Manager, ManagerConfigBuilder};

pub async fn create_manager() -> Result<Manager> {
    let manager = Manager::create_from_config(
        ManagerConfigBuilder::default()
            .database_url(std::env::var("DATABASE_URL").expect("未找到数据库配置"))
            .build()?,
    )
    .await?;

    Ok(manager)
}