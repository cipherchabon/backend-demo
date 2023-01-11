use std::net::TcpListener;

use actix_web::{dev::Server, get, App, HttpResponse, HttpServer, Responder};

// health_check endpoint
#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let app = || App::new().service(health_check);

    let server = HttpServer::new(app).listen(listener)?.run();

    Ok(server)
}
