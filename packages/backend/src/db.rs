use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
#[allow(dead_code)]
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create database pool")
}
