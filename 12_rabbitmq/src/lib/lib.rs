use clap::{Args};
use std::thread;
use std::time::Duration;
use log::{error, info, debug};


mod rabbit_client;

use rabbit_client::{RabbitClient, RabbitConParams};

#[derive(Debug, Args)]
pub struct PublisherArgs {
    /// Number of parallel publishings
    #[clap(long, short, default_value_t = 1)]
    pub threads: usize,

    /// Delay in miliseconds between the publishings
    #[clap(long, short, default_value_t = 100)]
    pub delay: usize,

    /// Number of bytes to publish
    #[clap(long, short = 'z', default_value_t = 1024)]
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

    /// Server to connect to
    #[clap(long, short, default_value = "localhost")]
    pub server: String,

    /// Port to connect to :5672
    #[clap(long, short, default_value_t = 5672)]
    pub port: u16,

    /// User name to authenticate
    #[clap(long, short)]
    pub user: String,

    /// Password to authenticate
    #[clap(long)]
    pub password: String,
}

fn create_con_params(args: GlobalOpts) -> RabbitConParams {
    RabbitConParams {
        server: args.server,
        port: args.port,
        user: args.user,
        password: args.password,
    }
}


pub async fn start_publisher(args: PublisherArgs) {
        println!("Hello, world ... I am a Publisher");
        let mut con = RabbitClient::new(create_con_params(args.global_opts));
        con.connect().await.unwrap();
        loop {
            let duration = Duration::from_secs(2);
            thread::sleep(duration);
            debug!("I am sleeping for 2s");
        }
}

pub async fn start_subscriber(args: SubscriberArgs) {
    println!("Hello, world ... I am a Subscriber");
    let mut con = RabbitClient::new(create_con_params(args.global_opts));
    con.connect().await.unwrap();
}
