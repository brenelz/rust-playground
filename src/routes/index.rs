use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
