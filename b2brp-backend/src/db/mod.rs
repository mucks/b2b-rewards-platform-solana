pub mod models;
pub mod models_create;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> DbPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
