use clap::Parser;
use std::sync::Arc;
use std::thread;


/// small http server.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of digits for the permutations
    #[arg(short, long, default_value_t = 8080)]
    pub port: usize,

    /// Address to bind the server
    #[arg(short, long, default_value = "0.0.0.0")]
    pub address: String,

    /// Number of requests before a worker thread is closed.
    /// Default value is 0, what disables the count.
    #[arg(short, long, default_value_t = 0)]
    pub count: usize,

    /// Number of worker threads.
    /// Default value is 1.
    #[arg(short, long, default_value_t = 1)]
    pub threads: usize,
}


pub fn run_server<'a> (args: &'a Args) {
    let max_request_count = args.count;
    let addr = args.address.clone();
    let port = args.port;
    let server_addr = format!("{}:{}", addr, port);
    let server = Arc::new(tiny_http::Server::http(server_addr).unwrap());
    println!("Server is bind to {} and listens on port {}", addr, port);

    let mut handles = Vec::new();

    for i in 0..args.threads {
        let server = server.clone();

        handles.push(thread::spawn(move || {
            println!("Thread-{} is waiting", i);
            let mut request_count: usize = 0;
            for rq in server.incoming_requests() {
                request_count += 1;
                let response_text = format!("hello world [thread #{}, request #{}]", i, request_count);
                let response = tiny_http::Response::from_string(response_text);
                let _ = rq.respond(response);
                if (max_request_count > 0) && (request_count == max_request_count) {
                    break;
                }
            }
            println!("Thread-{} is terminating", i);
        }));
    }

    println!("waiting for threads to terminate");
    for h in handles {
        h.join().unwrap();
    }
    println!("all threads are done :)");
}