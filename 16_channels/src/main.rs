//! Since common mpsc Rust channels only provide multiple sender and single receiver
//! crossbeam channels are used here

use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
struct Orchestrator {
    sender: Sender<i32>,
    receiver: Receiver<i32>,
}


impl Orchestrator {
    fn new() -> Orchestrator {
        let (s, r): (Sender<i32>, Receiver<i32>) = unbounded();
        Orchestrator { sender: s, receiver: r }
    }

    fn get_receiver(& self) -> Receiver<i32> {
        self.receiver.clone()
    }
}

struct Orchestrated {
    receiver: Receiver<i32>
}

impl Orchestrated {
    fn new(orchestrator: &Orchestrator) -> Orchestrated {
        Orchestrated {
            receiver: orchestrator.get_receiver()
        }
    }
}

fn main() {
    let o = Orchestrator::new(); 
    println!("Hello, world!");
}
