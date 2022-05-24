use std::{fs::read_dir, net::SocketAddr, path::PathBuf};

use anyhow::Result;
use axum::Extension;
use clap::Parser;
use inspirer_content::manager::{Manager, ManagerConfigBuilder};
use tracing_subscriber::EnvFilter;

use crate::route::create_routes;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    #[clap(short, long)]
    listen: Option<SocketAddr>,
    #[clap(short = 'D', long)]
    daemon: bool,
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    // 日志初始化
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async move { start_server(args).await })
}

async fn start_server(args: Args) -> Result<()> {
    let manager = Manager::create_from_config(
        ManagerConfigBuilder::default()
            .database_url(std::env::var("DATABASE_URL").expect("未找到数据库配置"))
            .build()?,
    )
    .await?;

    axum::Server::bind(&args.listen.unwrap_or("0.0.0.0:8088".parse()?))
        .serve(
            create_routes()
                .layer(Extension(manager))
                .into_make_service(),
        )
        .await?;

    Ok(())
}
