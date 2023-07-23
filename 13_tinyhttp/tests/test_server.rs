use my_http_impl::{Args, run_server};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;


#[test]
fn test_01() {
    let args = Args {
        port: 8080,
        address: "0.0.0.0".to_string(),
        count: 1,
        threads: 1,
    };

    let (sender, receiver) = channel(); 
    let a = args.address.clone();
    let p = args.port;

    thread::spawn(move || {
        run_server(&args);
        sender.send(()).unwrap();
    });

    thread::sleep(Duration::from_millis(500));

    let server_addr = format!("http://{}:{}", a, p);
    let response = ureq::get(&server_addr)
        .call().unwrap();

    assert_eq!(response.status(), 200, "server doesn't respond with 200");

    match receiver.recv_timeout(Duration::from_secs(10)) {
        Ok(_) => println!("run_server quit in time"),
        Err(_) => assert!(false, "run_server ran into timeout")
    }
}

#[test]
fn test_02() {
    let args = Args {
        port: 8081,
        address: "0.0.0.0".to_string(),
        count: 2,
        threads: 1,
    };

    let (sender, receiver) = channel(); 
    let a = args.address.clone();
    let p = args.port;

    thread::spawn(move || {
        run_server(&args);
        sender.send(()).unwrap();
    });

    thread::sleep(Duration::from_millis(500));

    let server_addr = format!("http://{}:{}", a, p);
    let response = ureq::get(&server_addr)
        .call().unwrap();

    assert_eq!(response.status(), 200, "server doesn't respond with 200");

    match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(_) => assert!(false, "run_server ran into timeout"),
        Err(_) => println!("server ran as expected in timeout")
    }
}

#[test]
fn test_03() {
    let args = Args {
        port: 8082,
        address: "0.0.0.0".to_string(),
        count: 2,
        threads: 1,
    };

    let (sender, receiver) = channel(); 
    let a = args.address.clone();
    let p = args.port;

    thread::spawn(move || {
        run_server(&args);
        sender.send(()).unwrap();
    });

    thread::sleep(Duration::from_millis(500));

    let server_addr = format!("http://{}:{}", a, p);
    let response = ureq::get(&server_addr)
        .call().unwrap();

    assert_eq!(response.status(), 200, "server doesn't respond with 200 (1)");

    let response2 = ureq::get(&server_addr)
        .call().unwrap();

    assert_eq!(response2.status(), 200, "server doesn't respond with 200 (2)");

    match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(_) => println!("run_server quit in time"),
        Err(_) => assert!(false, "run_server ran into timeout")
    }
}
