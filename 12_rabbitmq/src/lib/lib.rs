use clap::{Args};
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time;
use log::{error, info, debug};


mod easy_client;

use easy_client::{RabbitClient, RabbitConParams};

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

        // is used to inform the user of the client, that something went so wrong in the client, that it couldn't be handled internally
        let (tx_panic, mut rx_panic): (Sender<String>, Receiver<String>) = mpsc::channel(1);
        let mut con = RabbitClient::new(create_con_params(args.global_opts), tx_panic.clone(), 4);
        con.connect().await.unwrap();
        // let publisher = con.get_publisher(1).await.unwrap();
        // publisher.open_channel().await.unwrap();
        // let publisher2 = con.get_publisher(2).await.unwrap();
        // publisher2.open_channel().await.unwrap();
        // let publisher3 = con.get_publisher(3).await.unwrap();
        // publisher3.open_channel().await.unwrap();
        // let publisher4 = con.get_publisher(4).await.unwrap();
        // publisher4.open_channel().await.unwrap();
        // let publisher5 = con.get_publisher(5).await.unwrap();
        // publisher5.open_channel().await.unwrap();
        // let publisher6 = con.get_publisher(6).await.unwrap();
        // publisher6.open_channel().await.unwrap();

        for id in 0..1 {
            let publisher = con.get_publisher(id).await.unwrap();
            tokio::spawn(async move {
                info!("thread-{} is started", id);
                let sleep_time = time::Duration::from_secs(3);
                match publisher.open_channel().await {
                    Ok(()) => {
                        debug!("channel-{} successfully opened", id);
                    },
                    Err(_) => {
                        error!("channel-{} could not be opened, cancel", id);
                    }
                }
                loop {
                    publisher.say_hello();
                    thread::sleep(sleep_time);
                }
            });
        }


        // for id in 0..50 {
        //     let publisher = con.get_publisher().await;
        //     tokio::spawn(async move {
        //         info!("thread-{} is started", id);    
        //         let sleep_time = time::Duration::from_secs(1);
        //         let s = "hallo :)".to_string();
        //         let mut i  = 0;

        //         loop { 
        //             i += 1;
        //                 info!("thread-{} is publishing ...", id);    
        //                 match publisher.publish("test-exchange".to_string(), "test".to_string(), s.as_bytes().to_vec()).await {
        //                     Ok(()) => {
        //                         debug!("thread-{} published successfully", id);
        //                     },
        //                     Err(e) => {
        //                         error!("error while publishing: {}", e.to_string());
        //                     }
        //                 }
        //             if (i==10) {
        //                 break;
        //             }
        //             info!("thread-{} is going to sleep ({}) ...", id, i);
        //             thread::sleep(sleep_time);
        //         }
        //         info!("thread-{} ended", id);
        //     });
        // }
    
        debug!("I am running until I receive a panic request ... or get killed");
        if let Some(msg) = rx_panic.recv().await {
            error!("receive panic msg: {}", msg);
        }
        info!("close program");
}

pub async fn start_subscriber(args: SubscriberArgs) {
    println!("Hello, world ... I am a Subscriber");
    // is used to inform the user of the client, that something went so wrong in the client, that it couldn't be handled internally
    let (tx_panic, rx_panic): (Sender<String>, Receiver<String>) = mpsc::channel(1);
    let mut con = RabbitClient::new(create_con_params(args.global_opts), tx_panic.clone(), 10);
    con.connect().await.unwrap();
}
