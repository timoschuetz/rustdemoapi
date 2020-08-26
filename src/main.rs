use actix_web::{web, App, HttpRequest, HttpServer, Responder};
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

async fn health() -> impl Responder {
    format!("Ok")
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/hostname", web::get().to(check))
            .route("/health", web::get().to(health))
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}