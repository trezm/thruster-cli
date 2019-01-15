use lazy_static::*;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

lazy_static! {
    static ref connection_pool: Pool<ConnectionManager<PgConnection>> = {
      dotenv().ok();

      let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
      let manager = ConnectionManager::new(database_url);
      let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap();

      pool
    };
}

pub mod db {
  use diesel::prelude::*;
  use diesel::pg::PgConnection;
  use diesel::r2d2::ConnectionManager;
  use dotenv::dotenv;
  use std::env;
  use r2d2::PooledConnection;

  use crate::util::connection_pool;

  pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set");

    connection_pool.get()
      .expect(&format!("Error connecting to {}", database_url))
  }
}
