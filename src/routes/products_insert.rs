use crate::{domain::Product, AppState};
use actix_web::{get, web::Data, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Response<'a> {
    products: &'a Vec<Product>,
}

#[get("/products/insert")]
async fn get_products_insert(data: Data<AppState>) -> impl Responder {
    let mut products = data.inmemory_products.lock().unwrap();
    products.push(Product::new("inserted product".to_owned()));

    HttpResponse::Ok().json(Response {
        products: &products,
    })
}
