use structopt::StructOpt;

mod cli_app;

#[derive(StructOpt, Debug)]
#[structopt(name = "Inspirer Content CLI")]
struct Opt {
    /// 数据库地址
    #[structopt(short, long, default = "localhost")]
    host: String,
    /// 数据库用户名
    #[structopt(short, long, default = "root")]
    username: String,
    /// 数据库密码
    #[structopt(short, long, default = "root")]
    password: String,
    /// 数据库名称
    #[structopt(short, long, default = "inspirer_blog")]
    database: String,
}

fn main() {
    let opt: Opt = Opt::from_args();

    // todo
}