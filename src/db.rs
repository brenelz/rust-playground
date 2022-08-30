use actix_web::{error, web, Error};

use crate::domain::Product;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub enum Queries {
    GetProducts,
}

pub async fn execute(pool: &Pool, query: Queries) -> Result<Vec<Product>, Error> {
    let pool = pool.clone();

    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || match query {
        Queries::GetProducts => get_products(conn),
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

fn get_products(conn: Connection) -> Result<Vec<Product>, rusqlite::Error> {
    let mut statement = conn.prepare("SELECT title FROM products")?;

    statement
        .query_map([], |row| Ok(Product::new(row.get(0)?)))
        .and_then(Iterator::collect)
}
