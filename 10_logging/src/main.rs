use signal_hook::{consts::SIGUSR1, consts::SIGUSR2, iterator::Signals};
use log::{error, info, debug, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::{thread, time};
use std::io::Error;
use std::sync::mpsc::{self, Receiver};
use std::process;

use env_logger::filter::{Builder, Filter};


#[macro_use]
extern crate log;

use env_logger::Env;

enum Direction {
    UP,
    DOWN
}

fn change_sleep_seconds(direction: Direction, current_sleep_seconds: u64) -> (u64, Direction) {
    match direction {
        Direction::UP => {
            if current_sleep_seconds < 10 {
                (current_sleep_seconds + 1, Direction::UP)
            } else {
                (current_sleep_seconds - 1, Direction::DOWN)
            }
        },
        Direction::DOWN => {
            if current_sleep_seconds > 1 {
                (current_sleep_seconds - 1, Direction::DOWN)
            } else {
                (current_sleep_seconds + 1, Direction::UP)
            }
        }
    }
}


fn main() -> Result<(), Error> {
    let mut run = 1;
    let mut signals = Signals::new(&[SIGUSR1, SIGUSR2])?;
    let mut sleep_seconds = 5;
    let mut sleep_time = time::Duration::from_secs(sleep_seconds);

    let env = Env::default()
    .filter_or("MY_LOG_LEVEL", "trace");

    env_logger::init_from_env(env);
    println!("My pid is {}", process::id());

    // channel to communicate between the signal handler the endless loop
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut direction_sleep = Direction::UP;
        let mut log_level_error = false;
        for sig in signals.forever() {
            match sig {
                SIGUSR1 => {
                    debug!("Received SIGUSR1 -> change sleep_seconds: current sleep_seconds={}", sleep_seconds);
                    (sleep_seconds, direction_sleep) = change_sleep_seconds(direction_sleep, sleep_seconds);
                    info!("new sleep_seconds={}", sleep_seconds);
                    tx.send(sleep_seconds.clone()).unwrap();
                },
                SIGUSR2 => {
                    debug!("Received SIGUSR2: reload logger config");
                    if log_level_error {
                        log_level_error = false;
                        log::set_max_level(LevelFilter::Debug);
                        info!("log-level is DEBUG");
                    } else {
                        log_level_error = true;
                        log::set_max_level(LevelFilter::Error);
                        error!("log-level is ERROR");
                    }
                },
                _ => {
                    error!("Received other signal :-/ {:?}", sig);
                }
            }
        }
    }); 
    thread::spawn(move || {
        loop {
            trace!("Test-Trace");
            debug!("Test-Debug");
            info!("Test-Info");
            error!("Test-Error");
            thread::sleep(time::Duration::from_secs(2));
        }
    });
    loop {

        println!("Hello, world! ({})", run);
        loop {
            match rx.try_recv() {
                Ok(new_sleep_seconds) => {
                    debug!("Received new sleep_seconds :) ..., new_sleep_seconds={}", new_sleep_seconds);
                    sleep_time = time::Duration::from_secs(new_sleep_seconds);
                },
                _ => {
                    debug!("... sleep_time remains the same");
                    break;
                }
            }
        }
        info!("I will sleep now for: {}", sleep_time.as_secs());
        thread::sleep(sleep_time);
        run += 1;
    }
}
