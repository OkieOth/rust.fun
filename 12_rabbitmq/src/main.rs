use clap::Parser;

use clap::{Args, Subcommand};
use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};

use tokio_executor_trait;
use tokio_reactor_trait;


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

#[derive(Debug, Args)]
struct PublisherArgs {
    /// Number of parallel publishings
    #[clap(long, short, default_value_t = 1)]
    threads: usize,

    /// Delay in miliseconds between the publishings
    #[clap(long, short, default_value_t = 100)]
    delay: usize,

    /// Number of bytes to publish
    #[clap(long, short = 's', default_value_t = 1024)]
    package_size: usize,

    #[clap(flatten)]
    global_opts: GlobalOpts,

}

#[derive(Debug, Args)]
struct SubscriberArgs {
    /// Number of parallel subscriptions
    #[clap(long, short, default_value_t = 1)]
    threads: usize,

    /// Prefetch count for the subscriptions
    #[clap(long, short = 'f', default_value_t = 1)]
    prefetch_count: usize,

    #[clap(flatten)]
    global_opts: GlobalOpts,
}


#[derive(Debug, Args)]
struct GlobalOpts {
    /// Exchange to use
    #[clap(long, short)]
    exchange: String,

    /// Routing-key to use
    #[clap(long, short)]
    routing_key: String,

    /// Connection string
    #[clap(long, short, default_value = "amqp://localhost:5672")]
    connection: String,

    /// User name to authenticate
    #[clap(long, short)]
    user: String,

    /// Password to authenticate
    #[clap(long, short)]
    password: String,
}

#[tokio::main]
async fn main() {
    let parser = App::parse();
    match parser.command {
        Command::Subscriber(_args) => {
            println!("Hello, world ... I am a Subscriber");
        },
        Command::Publisher(_args) => {
            println!("Hello, world ... Call me publisher");
        },
    }
}

async fn get_connection<'a> (user: &'a String, password: &'a String, connection: &'a String) -> Connection {
    let options = ConnectionProperties::default()
        // Use tokio executor and reactor.
        // At the moment the reactor is only available for unix.
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    Connection::connect(connection, options).await.unwrap()
}

