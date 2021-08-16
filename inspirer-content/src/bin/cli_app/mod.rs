mod command;

use sqlx::MySqlPool;
use sqlx::mysql::MySqlConnectOptions;
use rustyline::{Editor, Config};
use rustyline::error::ReadlineError;
use crate::cli_app::command::create;

pub struct Application(pub MySqlPool);

impl Application {
    pub async fn new(hostname: &str, port: u16, username: &str, password: &str, database: &str) -> sqlx::Result<Self> {
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

        Ok(Application(pool))
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::builder()
            .history_ignore_space(true)
            .build();

        let mut rl = Editor::<()>::with_config(config);

        loop {
            let input = rl.readline("> ");
            match input {
                Ok(command) => {
                    match command.as_str() {
                        "create" => {
                            create(self).await?;
                        }
                        _ => ()
                    }
                }
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => (),
                Err(err) => break,
            }
        }

        Ok(())
    }
}