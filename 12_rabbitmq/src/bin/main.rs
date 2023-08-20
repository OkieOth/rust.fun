use log::{error, info, debug, LevelFilter};
use env_logger::filter::{Builder, Filter};
use env_logger::Env;

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


fn main() {
    let env = Env::default()
    .filter_or("LOG_LEVEL", "info");

    env_logger::init_from_env(env);


    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on( async {
            let parser = App::parse();
            match parser.command {
                Command::Subscriber(args) => {
                    start_subscriber(args).await;
                },
                Command::Publisher(args) => {
                    start_publisher(args).await;
                },
            };
            }
        );
}
