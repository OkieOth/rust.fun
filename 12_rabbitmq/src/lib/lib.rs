use clap::{Args};


mod rabbit_client;

use rabbit_client::RabbitConnection;

#[derive(Debug, Args)]
pub struct PublisherArgs {
    /// Number of parallel publishings
    #[clap(long, short, default_value_t = 1)]
    pub threads: usize,

    /// Delay in miliseconds between the publishings
    #[clap(long, short, default_value_t = 100)]
    pub delay: usize,

    /// Number of bytes to publish
    #[clap(long, short = 's', default_value_t = 1024)]
    pub package_size: usize,

    #[clap(flatten)]
    pub global_opts: GlobalOpts,

}

#[derive(Debug, Args)]
pub struct SubscriberArgs {
    /// Number of parallel subscriptions
    #[clap(long, short, default_value_t = 1)]
    pub threads: usize,

    /// Prefetch count for the subscriptions
    #[clap(long, short = 'f', default_value_t = 1)]
    pub prefetch_count: usize,

    #[clap(flatten)]
    pub global_opts: GlobalOpts,
}


#[derive(Debug, Args)]
pub struct GlobalOpts {
    /// Exchange to use
    #[clap(long, short)]
    pub exchange: String,

    /// Routing-key to use
    #[clap(long, short)]
    pub routing_key: String,

    /// Connection string
    #[clap(long, short, default_value = "amqp://localhost:5672")]
    pub connection: String,

    /// User name to authenticate
    #[clap(long, short)]
    pub user: String,

    /// Password to authenticate
    #[clap(long, short)]
    pub password: String,
}


pub async fn start_publisher(args: PublisherArgs) {
    println!("Hello, world ... I am a Publisher");
    let mut con = RabbitConnection::new();
    con.connect().await.unwrap();
}

pub async fn start_subscriber(args: SubscriberArgs) {
    println!("Hello, world ... I am a Subscriber");
    let mut con = RabbitConnection::new();
    con.connect().await.unwrap();
}
