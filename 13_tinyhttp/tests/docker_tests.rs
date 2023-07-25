#[test]
#[ignore]
fn docker_test_01() {
    println!("I am only used in docker compose based integration tests");
    let server_addr = format!("http://my_http_server:8080");
    let response = ureq::get(&server_addr)
        .call().unwrap();

    assert_eq!(response.status(), 200, "server doesn't respond with 200");

}

#[test]
#[ignore]
fn test_01() {
    println!("I want also be executed in docker :-/");
    assert!(false, "I want also be executed in docker :-/");
}
