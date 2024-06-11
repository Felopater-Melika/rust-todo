use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> Pool {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create pool.")
}