use log::{error, info, debug, LevelFilter};

use clap::Parser;

use clap::{Subcommand};


use rabbit_client_impl::{SubscriberArgs, PublisherArgs, start_publisher, start_subscriber};


// https://rust-cli-recommendations.sunshowers.io/handling-arguments.html


/// Here's my app!
#[derive(Debug, Parser)]
#[clap(name = "rabbitmq-test", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Use the program as a subscriber.
    Subscriber(SubscriberArgs),

    /// Use the program as publisher
    Publisher(PublisherArgs),
}


#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let parser = App::parse();
    match parser.command {
        Command::Subscriber(args) => {
            match start_subscriber(args) {
                Err(s) => {
                    error!("Error while run subscriber: {}", s);
                },
                _ => {}
            }
        },
        Command::Publisher(args) => {
            match start_publisher(args) {
                Err(s) => {
                    error!("Error while run publisher: {}", s);
                },
                _ => {}
            }
        },
    }
}
