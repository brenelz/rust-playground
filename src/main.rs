use actix_web::{web::Data, App, HttpServer};
use domain::Product;
use r2d2_sqlite::SqliteConnectionManager;
use routes::{get_index, get_products, get_products_fromdb, get_products_insert};
use std::sync::Mutex;

mod db;
mod domain;
mod routes;

#[derive(Debug)]
struct AppState {
    inmemory_products: Mutex<Vec<Product>>,
    dbpool: db::Pool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = SqliteConnectionManager::file("db/products.db");
    let dbpool = db::Pool::new(manager).unwrap();

    println!("Listening on port 8080");

    HttpServer::new(move || {
        let app_state = AppState {
            dbpool: dbpool.clone(),
            inmemory_products: Mutex::new(vec![
                Product::new("test123".to_owned()),
                Product::new("test12345".to_owned()),
            ]),
        };

        App::new()
            .app_data(Data::new(app_state))
            .service(get_index)
            .service(get_products)
            .service(get_products_fromdb)
            .service(get_products_insert)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
