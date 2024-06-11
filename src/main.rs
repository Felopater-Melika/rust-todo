#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};

mod services;
mod utils;
mod db;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let pool = db::connection::establish_connection_pool();
    HttpServer::new(move || {
        App::new().app_data(pool.clone())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
