use axum::prelude::*;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::MySqlPool;
use axum::AddExtensionLayer;
use structopt::StructOpt;
use std::net::SocketAddr;
use std::str::FromStr;
use axum::extract::Extension;

mod service_server;

#[derive(StructOpt, Debug)]
#[structopt(name = "Inspirer Content CLI")]
struct Opt {
    /// 数据库地址
    #[structopt(short, long, default_value = "localhost")]
    host: String,
    /// 数据库端口
    #[structopt(short, long, default_value = "3306")]
    port: u16,
    /// 数据库用户名
    #[structopt(short, long, default_value = "root")]
    username: String,
    /// 数据库密码
    #[structopt(short = "P", long, default_value = "root")]
    password: String,
    /// 数据库名称
    #[structopt(short, long, default_value = "inspirer_blog")]
    database: String,
    /// 监听地址
    #[structopt(short, long, default_value = "127.0.0.1:8802")]
    listen: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt: Opt = Opt::from_args();

    let pool = database_provider(
        opt.host.as_str(),
        opt.port,
        opt.username.as_str(),
        opt.password.as_str(),
        opt.database.as_str(),
    ).await?;

    let app = route(
        "/content",
        post(service_server::create)
            .get(service_server::list_simple),
    )
        .route("/content/:id", get(service_server::find))
        .layer(AddExtensionLayer::new(pool));

    axum::Server::bind(&SocketAddr::from_str(opt.listen.as_str()).unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn database_provider(hostname: &str, port: u16, username: &str, password: &str, database: &str) -> anyhow::Result<MySqlPool> {
    let options = MySqlConnectOptions::new()
        .host(hostname)
        .username(username)
        .password(password)
        .database(database)
        .port(port);

    let pool = MySqlPool::connect_with(options)
        .await?;

    // Test pool
    let _ = pool.acquire().await?;

    Ok(pool)
}