use crate::{
    db::{self, Queries},
    domain::Product,
    AppState,
};
use actix_web::{get, web::Data, Error as AWError, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Response<'a> {
    products: &'a Vec<Product>,
}

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/products")]
async fn get_products(data: Data<AppState>) -> impl Responder {
    let products = data.inmemory_products.lock().unwrap();
    HttpResponse::Ok().json(Response {
        products: &products,
    })
}

#[get("/products/fromdb")]
async fn get_products_fromdb(data: Data<AppState>) -> Result<HttpResponse, AWError> {
    let products: Vec<Product> = db::execute(&data.dbpool, Queries::GetProducts).await?;

    Ok(HttpResponse::Ok().json(Response {
        products: &products,
    }))
}

#[get("/products/insert")]
async fn get_products_insert(data: Data<AppState>) -> impl Responder {
    let mut products = data.inmemory_products.lock().unwrap();
    products.push(Product::new("inserted product".to_owned()));

    HttpResponse::Ok().json(Response {
        products: &products,
    })
}
