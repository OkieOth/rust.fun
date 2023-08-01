use clap::{Args};

#[derive(Debug, Args)]
pub struct ServerArgs {
    /// Number of threads to run the server
    #[clap(long, short, default_value_t = 1)]
    pub threads: usize,

    /// Port to listen
    #[arg(short, long, default_value_t = 8080)]
    pub port: usize,

    /// Address to bind the server
    #[arg(short, long, default_value = "0.0.0.0")]
    pub address: String,

    /// Number of requests before a worker thread is closed.
    /// Default value is 0, what disables the count.
    #[arg(short, long, default_value_t = 0)]
    pub count: usize,

    /// Number of bytes in the response
    #[arg(short, long, default_value_t = 1024)]
    pub response_size: usize,
}

#[derive(Debug, Args)]
pub struct ClientArgs {
    /// Number of threads to challenge the server in parallel
    #[clap(long, short, default_value_t = 1)]
    pub threads: usize,

    /// Port to listen
    #[arg(short, long, default_value_t = 8080)]
    pub port: usize,

    /// Address to bind the server
    #[arg(short, long, default_value = "0.0.0.0")]
    pub address: String,

    /// Number of requests before a worker thread is closed.
    /// Default value is 0, what disables the count.
    #[arg(short, long, default_value_t = 1)]
    pub count: usize,
}

pub fn start_server(args: ServerArgs) -> Result<(), &'static str> {
    println!("Hello, world ... I am a Server");
    Ok(())
}

pub fn start_client(args: ClinetArgs) -> Result<(), &'static str> {
    println!("Hello, world ... I am a Client");
    Ok(())
}
