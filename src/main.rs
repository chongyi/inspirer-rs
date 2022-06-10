use clap::Parser;

mod cli;
mod controller;
mod error;
mod request;
mod response;
mod route;
mod server;

fn main() {
    let cli = cli::Cli::parse();

    dotenv::dotenv().expect("Initialize dotenv error.");

    cli.run();

    std::process::exit(0);
}
