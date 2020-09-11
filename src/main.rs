use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::collections::HashMap;
extern crate sys_info;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn check() -> impl Responder {
    let hostname = sys_info::hostname();
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    format!("{:?} - {:?}", VERSION, hostname)
}

async fn tr_fib(req: HttpRequest) -> impl Responder {
    let zahl :&str = req.match_info().get("zahl").unwrap_or("1");
    let zahli :i32 = zahl.parse::<i32>().unwrap();
    let hostname = sys_info::hostname();
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let mut kv_map = HashMap::<i32, i128>::new();
    format!("Server: {:?} on {} - Fibonacci Zahl von {} ist {}", hostname, VERSION, zahli, fibonacci(zahli, &mut kv_map))
}

fn fibonacci(zahl: i32, kv_map: &mut HashMap<i32, i128>) -> i128 {
    return if zahl == 1 || zahl == 2 {
        1
    } else if zahl > 0 {
            match kv_map.get(&zahl) {
                Some(&number) => return number,
                _ => {
                        let res = fibonacci(zahl - 1, kv_map) + (fibonacci(zahl - 2, kv_map));
                        // println!("{:?}", res);
                        // println!("Before insert: {:?}", kv_map);
                        kv_map.insert(zahl, res);
                        // println!("After insert: {:?}", kv_map);
                        return res;
                },
            };
    } else {
        0
    }
}

async fn health() -> impl Responder {
    format!("Ok")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Server started");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/hostname", web::get().to(check))
            .route("/health", web::get().to(health))
            .route("/{name}", web::get().to(greet))
            .route("/fibo/{zahl}", web::get().to(tr_fib))
    })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
