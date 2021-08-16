use structopt::StructOpt;
use cli_app::Application;

pub mod cli_app;

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();

    let application = Application::new(
        opt.host.as_str(),
        opt.port,
        opt.username.as_str(),
        opt.password.as_str(),
        opt.database.as_str(),
    ).await?;

    application.run().await
}