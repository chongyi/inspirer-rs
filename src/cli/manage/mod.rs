use clap::{Args, Subcommand};
use tokio::runtime::Runtime;

use crate::manager::create_manager;

use self::user::UserManage;

pub mod user;


#[derive(Args)]
pub struct  Manage {
    #[clap(subcommand)]
    commands: ManageCommands
}

#[derive(Subcommand)]
pub enum ManageCommands {
    User(UserManage)
}

impl Manage {
    pub fn run(self) {
        let rt = Runtime::new().expect("创建运行时失败");

        rt.block_on(async move {
            let manager = create_manager().await.expect("创建 Manager 失败");

            match self.commands {
                ManageCommands::User(command) => command.run(manager).await,
            }
        });
    }
}
