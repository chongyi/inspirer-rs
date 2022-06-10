use std::net::SocketAddr;

use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[clap(version, about)]
pub struct Cli {
    #[clap(short, long)]
    pub listen: Option<SocketAddr>,
    #[clap(short = 'D', long)]
    pub daemon: bool,
    #[clap(subcommand)]
    commands: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Manage(Manage)
}

#[derive(Args)]
pub struct  Manage {
    #[clap(subcommand)]
    commands: ManageCommands
}

#[derive(Subcommand)]
pub enum ManageCommands {
    User(UserManage)
}

#[derive(Args)]
pub struct UserManage {
    #[clap(subcommand)]
    commands: UserManageCommands,
}

#[derive(Subcommand)]
pub enum UserManageCommands {
    Create {
        username: String,
        nickname: Option<String>,
    }
}

impl Cli {
    pub fn run(self) {
        match self.commands {
            Some(commands) => {},
            None => crate::server::run(self).unwrap()
        }
    }
}