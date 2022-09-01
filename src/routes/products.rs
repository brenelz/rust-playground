use crate::{domain::Product, AppState};
use actix_web::{get, web::Data, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Response<'a> {
    products: &'a Vec<Product>,
}

#[get("/products")]
async fn get_products(data: Data<AppState>) -> impl Responder {
    let products = data.inmemory_products.lock().unwrap();
    HttpResponse::Ok().json(Response {
        products: &products,
    })
}
