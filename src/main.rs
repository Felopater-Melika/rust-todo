#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, web};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use tracing_subscriber::fmt;

pub mod services;
pub mod utils;
pub mod db;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
    let pool = Pool::builder(config).build().expect("Failed to create pool");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/users")
                    .route(web::get().to(services::services::get_all_users))
                    .route(web::post().to(services::services::create_user))
            )
            .service(
                web::resource("/users/{id}")
                    .route(web::get().to(services::services::get_user_by_id))
                    .route(web::delete().to(services::services::delete_user))
                    .route(web::put().to(services::services::update_user))
            )
            .service(
                web::resource("/todos")
                    .route(web::get().to(services::services::get_all_todos))
                    .route(web::post().to(services::services::create_todo))
            )
            .service(
                web::resource("/todos/{id}")
                    .route(web::get().to(services::services::get_todo_by_id))
                    .route(web::delete().to(services::services::delete_todo))
                    .route(web::put().to(services::services::update_todo))
            )
            .service(
                web::resource("/users/{id}/todos")
                    .route(web::get().to(services::services::get_all_todos_for_user))
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}