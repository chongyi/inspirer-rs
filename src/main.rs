use std::process::ExitCode;

mod controller;
mod error;
mod request;
mod response;
mod route;
mod server;

fn main() {
    dotenv::dotenv().expect("Initialize dotenv error.");

    #[cfg(target_family = "unix")]
    {
        let daemonize = std::env::var("DAEMONIZE")
            .map(|s| s == "true" || s == "1")
            .unwrap_or(false);

        if daemonize {
            let daemon = Daemon::new()
                .start();

            match daemon {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(ExitCode::FAILURE);
                }
            }
        }
    
    }

    server::run().unwrap();

    std::process::exit(ExitCode::SUCCESS);
}
