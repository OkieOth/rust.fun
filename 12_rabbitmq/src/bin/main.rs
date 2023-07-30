use log::{error, info, debug, LevelFilter};

use clap::Parser;

use clap::{Subcommand};
// use lapin::{
//     message::DeliveryResult,
//     options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
//     types::FieldTable,
//     BasicProperties, Connection, ConnectionProperties,
// };

use tokio_executor_trait;
use tokio_reactor_trait;

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


#[tokio::main]
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

// async fn get_connection<'a> (user: &'a String, password: &'a String, connection: &'a String) -> Connection {
//     let options = ConnectionProperties::default()
//         // Use tokio executor and reactor.
//         // At the moment the reactor is only available for unix.
//         .with_executor(tokio_executor_trait::Tokio::current())
//         .with_reactor(tokio_reactor_trait::Tokio);

//     Connection::connect(connection, options).await.unwrap()
// }

