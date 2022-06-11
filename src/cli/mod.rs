use std::net::SocketAddr;

use clap::{Parser, Subcommand, Args};

use self::manage::Manage;

pub mod manage;

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


impl Cli {
    pub fn run(self) {
        match self.commands {
            Some(commands) => {
                match commands {
                    Commands::Manage(manage) => {
                        println!("=> Run manage");
                        manage.run()
                    }
                }
            },
            None => crate::server::run(self).unwrap()
        }
    }
}
