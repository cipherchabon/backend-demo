use actix_web::{get, HttpResponse, Responder};

// health_check endpoint
#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
