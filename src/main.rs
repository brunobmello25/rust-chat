use chat::{client::Client, server::Server};
use config::Config;

mod chat;
mod config;

fn main() -> Result<(), &'static str> {
    let config = Config::from_args(std::env::args())?;

    match config.mode {
        config::Mode::Server => run_server(),
        config::Mode::Client => run_client(),
    }

    Ok(())
}

fn run_server() {
    let server = Server::new();

    server.run();
}

fn run_client() {
    let client = Client::new();

    client.run();
}
