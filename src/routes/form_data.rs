use actix_web::{post, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct FormData {
    name: String,
}

#[post("/formData")]
async fn post_form_data(form: Json<FormData>) -> impl Responder {
    HttpResponse::Ok().body(form.0.name)
}
