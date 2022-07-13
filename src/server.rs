use anyhow::Result;
use axum::Extension;
use tracing_subscriber::EnvFilter;

use crate::{cli::Cli, route::create_routes, manager::create_manager};

pub fn run(args: Cli) -> Result<()> {
    #[cfg(target_family = "unix")]
    {
        let daemonize = std::env::var("DAEMONIZE")
            .map(|s| s == "true" || s == "1")
            .unwrap_or(false);

        if daemonize {
            let daemon = daemonize_me::Daemon::new().start();

            match daemon {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    // 日志初始化
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async move { start_server(args).await })
}

async fn start_server(args: Cli) -> Result<()> {
    let manager = create_manager().await?;

    axum::Server::bind(&args.listen.unwrap_or("0.0.0.0:8088".parse()?))
        .serve(
            create_routes()
                .layer(Extension(manager))
                .into_make_service(),
        )
        .await?;

    Ok(())
}
