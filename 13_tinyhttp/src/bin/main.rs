extern crate tiny_http;

// insprired/copied from https://github.com/tiny-http/tiny-http/blob/master/examples/hello-world.rs


use my_http_impl::{Args, run_server};

use clap::Parser;


fn main() {
    let args = <Args as Parser>::parse();
    run_server(&args);
}