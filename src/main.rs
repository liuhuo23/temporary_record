mod model;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body("0.1.0")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).service(version))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
