use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;
use std::env;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub fn get_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
