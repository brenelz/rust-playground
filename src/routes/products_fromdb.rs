use crate::{
    db::{self, Queries},
    domain::Product,
    AppState,
};
use actix_web::{get, web::Data, Error as AWError, HttpResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Response<'a> {
    products: &'a Vec<Product>,
}

#[get("/products/fromdb")]
async fn get_products_fromdb(data: Data<AppState>) -> Result<HttpResponse, AWError> {
    let products: Vec<Product> = db::execute(&data.dbpool, Queries::GetProducts).await?;

    Ok(HttpResponse::Ok().json(Response {
        products: &products,
    }))
}
