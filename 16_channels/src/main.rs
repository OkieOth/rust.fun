use env_logger::filter::{Builder, Filter};
use env_logger::Env;
use log::{debug, error, info, LevelFilter};
use std::thread;
use std::time;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

struct Test {}

#[tokio::main]
async fn main() {
    let env = Env::default().filter_or("LOG_LEVEL", "info");

    env_logger::init_from_env(env);

    let mut writers: Vec<Sender<Test>> = Vec::new();
    for id in 1..10 {
        let (tx, mut rx): (Sender<Test>, Receiver<Test>) = mpsc::channel(1);
        writers.push(tx);
        tokio::spawn(async move {
            info!("thread-{} started.", id);
            for i in 0..id {
                match rx.recv().await {
                    Some(t) => {
                        info!("thread-{} received a package.", id)
                    }
                    None => {
                        info!("thread-{} received nix :-(", id)
                    }
                }
            }
            info!("thread-{} ended.", id);
        });
    }
    let sleep_time = time::Duration::from_secs(1);

    info!("start sending to threads ...");
    loop {
        let mut elems_to_del: Vec<usize> = Vec::new();
        let mut index: usize = 0;

        if writers.is_empty() {
            break;
        }

        info!("send thread starts sending ...");
        for sender in writers.iter() {
            match sender.send(Test {}).await {
                Ok(_) => {
                    info!("successfully send to thread-{}", index);
                }
                Err(e) => {
                    elems_to_del.push(index);
                    error!("error while send to thread-{}: {}", index, e.to_string());
                }
            }
            index += 1;
        }
        // remove sender with errors
        for di in elems_to_del.iter() {
            writers.remove(*di);
        }
        info!("send thread starts sleeping ...");
        thread::sleep(sleep_time);
        info!("send thread is awake");
    }
}
