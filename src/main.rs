use actix_web::{get, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    title: String,
    price: f64,
}

#[get("/")]
async fn index() -> HttpResponse {
    let index = [
        Product {
            title: "Product1".to_string(),
            price: 3.00,
        },
        Product {
            title: "Product2".to_string(),
            price: 3.00,
        },
    ];
    HttpResponse::Ok().json(index)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on port 8080");
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
