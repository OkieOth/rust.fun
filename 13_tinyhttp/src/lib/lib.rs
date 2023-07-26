use clap::Parser;
use tiny_http::{Request, Response};
use std::sync::Arc;
use std::thread;
use std::io::{Cursor};
use regex::{Regex, Captures};
use log::{error, info, debug, LevelFilter, Log, Metadata, Record, SetLoggerError};

#[macro_use]
extern crate log;

use env_logger::Env;


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

fn handle_request(i: usize, request_count: usize) -> Response<Cursor<Vec<u8>>> {
    trace!("Test-Trace: thread-{}, request_count={}", i, request_count);
    debug!("Test-Debug: thread-{}, request_count={}", i, request_count);
    info!("Test-Info: thread-{}, request_count={}", i, request_count);
    warn!("Test-Warn: thread-{}, request_count={}", i, request_count);
    error!("Test-Error: thread-{}, request_count={}", i, request_count);
    let response_text = format!("hello world [thread #{}, request #{}]", i, request_count);
    tiny_http::Response::from_string(response_text)
}

fn set_log_level(captures: &Captures) -> Response<Cursor<Vec<u8>>> {
    let log_level = captures.get(1).unwrap().as_str();
    match log_level {
        "error" => {
            log::set_max_level(LevelFilter::Error);
            warn!("set log-level to ERROR");
        },
        "warn" => {
            log::set_max_level(LevelFilter::Warn);
            warn!("set log-level to WARN");
        },
        "info" => {
            log::set_max_level(LevelFilter::Info);
            warn!("set log-level INFO");
        },
        "debug" => {
            log::set_max_level(LevelFilter::Debug);
            warn!("set log-level DEBUG");
        },
        "trace" => {
            log::set_max_level(LevelFilter::Trace);
            warn!("set log-level TRACE");
        },
        _ => error!("receive wish to switch to unknown log-level: {}", log_level)
    }
    tiny_http::Response::from_string("new log-level set")
}

pub fn run_server<'a> (args: &'a Args) {
    let max_request_count = args.count;
    let addr = args.address.clone();
    let port = args.port;
    let server_addr = format!("{}:{}", addr, port);
    let server = Arc::new(tiny_http::Server::http(server_addr).unwrap());

    let env = Env::default().filter_or("LOG_LEVEL", "info");
    env_logger::init_from_env(env);


    info!("Server is bind to {} and listens on port {}, max_request_count={}", addr, port, max_request_count);

    let mut handles = Vec::new();

    for i in 0..args.threads {
        let server = server.clone();

        handles.push(thread::spawn(move || {
            debug!("Thread-{} is waiting", i);
            let re = Regex::new(r"/log-level/(.*)$").unwrap();
            let mut request_count: usize = 0;
            for rq in server.incoming_requests() {
                let url = rq.url();
                info!("got request for: {}",url);
                match re.captures(url) {
                    Some(captures) => {
                        let r = set_log_level(&captures);
                        let _ = rq.respond(r);
                    },
                    None => {
                        request_count += 1;
                        let _ = rq.respond(handle_request(i, request_count));
                        if (max_request_count > 0) && (request_count == max_request_count) {
                            break;
                        }
                    }
                }
            }
            debug!("Thread-{} is terminating", i);
        }));
    }

    debug!("waiting for threads to terminate");
    for h in handles {
        h.join().unwrap();
    }
    debug!("all threads are done :)");
}