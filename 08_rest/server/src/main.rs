use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use std::fmt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello_01)
            .service(hello_02)
            .service(hello_03)
            .service(hello_04)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello_01() -> impl Responder {
    HttpResponse::Ok().body("Hello world (1)!")
}

// https://actix.rs/docs/extractors

#[get("/hello02/world/{part1}/{part2}")]
async fn hello_02(path: web::Path<(String, String)>) -> impl Responder {
    let (part1, part2) = path.into_inner();
    HttpResponse::Ok().body(format!("Hello world (2): part1={}, part2={}", part1, part2))
}

#[get("/hello03/world/{part1}/{part2}/{id}")]
async fn hello_03(path: web::Path<(String, String, u32)>) -> impl Responder {
    let (part1, part2, id) = path.into_inner();
    HttpResponse::Ok().body(format!("Hello world (2): part1={}, part2={}, id={}", part1, part2, id))
}

// wget http://localhost:8080/hello04/world/berlin/friedrichshain?id=300&start=2023-06-26
#[get("/hello04/world/{part1}/{part2}")]
async fn hello_04(path: web::Path<(String, String)>,
    query: web::Query<MyPageQueryParams>) -> impl Responder {
    //let (part1, part2) = path.into_inner();
    let query_params = query.into_inner();
    let (part1, part2) = path.into_inner();
    HttpResponse::Ok().body(format!("Hello world (4): part1={}, part2={}, {}", part1, part2, query_params))
}

#[derive(Deserialize)]
struct MyPageQueryParams {
    id: Option<String>,
    start: Option<String>,
}

impl std::fmt::Display for MyPageQueryParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the username if present
        let s = "???".to_string();
        let id_str: &String;
        let start_str: &String;
        if self.id.is_some() {
            id_str = self.id.as_ref().unwrap();
        } else {
            id_str = &s;
        }
        if self.start.is_some() {
            start_str = self.start.as_ref().unwrap();
        } else {
            start_str = &s;
        }
        write!(f, "id={}, start={}", id_str, start_str);
        Ok(())
    }
}

