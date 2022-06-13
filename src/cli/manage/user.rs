use clap::{Args, Subcommand};
use inspirer_content::{manager::Manager, model, service::user::UserService};

#[derive(Args)]
pub struct UserManage {
    #[clap(subcommand)]
    commands: UserManageCommands,
}

#[derive(Subcommand)]
pub enum UserManageCommands {
    Create {
        #[clap(short, long)]
        username: String,
        #[clap(short, long)]
        nickname: Option<String>,
        #[clap(short, long)]
        password: Option<String>,
    },
}

impl UserManage {
    pub async fn run(self, manager: Manager) {
        match self.commands {
            UserManageCommands::Create { username, nickname, password } => {
                println!("=> Create user");
                println!("-> Username = {username}");
                
                let (uuid, pkey) = manager
                    .create_user_simple(model::user::NewUser {
                        username,
                        nickname: nickname.unwrap_or_default(),
                        password: password.unwrap_or_default(),
                        avatar: Default::default(),
                    })
                    .await
                    .expect("创建用户失败");

                println!("-> UUID = {uuid}");
                println!("-> Private key:");
                println!("{pkey}");
            }
        }
    }
}
