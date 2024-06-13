use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
