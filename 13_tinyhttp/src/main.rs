extern crate tiny_http;

// insprired/copied from https://github.com/tiny-http/tiny-http/blob/master/examples/hello-world.rs

use std::sync::Arc;
use std::thread;

fn main() {
    let server = Arc::new(tiny_http::Server::http("0.0.0.0:9975").unwrap());
    println!("Now listening on port 9975");

    let mut handles = Vec::new();

    for i in 0..4 {
        let server = server.clone();

        handles.push(thread::spawn(move || {
            println!("Thread-{} is waiting", i);
            for rq in server.incoming_requests() {
                let response = tiny_http::Response::from_string("hello world".to_string());
                let _ = rq.respond(response);
                break;
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