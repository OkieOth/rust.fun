use log::{error, info, debug, LevelFilter};

use clap::Parser;

use clap::{Subcommand};

/// Simple programm that implements a async udp server and clients
#[derive(Debug, Parser)]
#[clap(name = "my_udp_worker", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Use the program as UDP server.
    Server(ServerArgs),

    /// Use the program as UDP client
    Client(ClientArgs),
}


#[tokio::main]
async fn main() {
    let parser = App::parse();
    match parser.command {
        Command::Server(args) => {
            match start_server(args) {
                Err(s) => {
                    error!("Error while run server: {}", s);
                },
                _ => {}
            }
        },
        Command::Client(args) => {
            match start_client(args) {
                Err(s) => {
                    error!("Error while run client: {}", s);
                },
                _ => {}
            }
        },
    }
}
