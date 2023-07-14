use clap::Parser;

/// Simple Rabbitmq Publisher/Subscriber
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// If true, then this program acts as publisher. Either publisher or subscriber
    #[arg(short, long, default_value_t = false)]
    publisher: bool,

    /// If true, then this program acts as subscriber
    #[arg(short, long, default_value_t = false)]
    subscriber: bool,
}


fn main() {
    println!("Hello, world!");
}
