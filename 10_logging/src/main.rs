use signal_hook::{consts::SIGUSR1, consts::SIGUSR2, iterator::Signals};
use log::{error, info, trace};
use log4rs;
use serde_yaml;
use std::{thread, time};
use std::io::Error;
use std::sync::mpsc;
use std::process;

fn main() -> Result<(), Error> {
    let mut run = 1;
    let mut signals = Signals::new(&[SIGUSR1, SIGUSR2])?;
    let mut sleep_seconds = 5;
    let mut sleep_time = time::Duration::from_secs(sleep_seconds);

    println!("My pid is {}", process::id());

    // channel to communicate between the signal handler the endless loop
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                SIGUSR1 => {
                    sleep_seconds += 1;
                    println!("Received SIGUSR1: sleep_seconds={}", sleep_seconds);
                    tx.send(sleep_seconds.clone()).unwrap();
                },
                SIGUSR2 => {
                    if sleep_seconds > 1 {
                        sleep_seconds -= 1;
                        println!("Received SIGUSR2: sleep_seconds= {}", sleep_seconds);
                        tx.send(sleep_seconds.clone()).unwrap();
                    } else {
                        println!("Received SIGUSR2: can't decrease, sleep_seconds reached minimum of 1 s");
                    }
                },
                _ => {
                    println!("Received other signal :-/ {:?}", sig);
                }
            }
        }
    });
    loop {
        println!("Hello, world! ({})", run);
        match rx.try_recv() {
            Ok(new_sleep_seconds) => {
                println!("Received new sleep_seconds :) ..., new_sleep_seconds={}", new_sleep_seconds);
                sleep_time = time::Duration::from_secs(new_sleep_seconds);
            },
            _ => {
                println!("... sleep_time remains the same");
            }
        }
        thread::sleep(sleep_time);
        run += 1;
    }
}
