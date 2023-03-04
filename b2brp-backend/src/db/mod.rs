pub mod models;
pub mod models_create;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
