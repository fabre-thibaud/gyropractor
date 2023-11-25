
mod db;
mod error;
mod handler;
mod model;
mod repo;
mod routes;

use dotenv::dotenv;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;
use warp::Rejection;

pub type Result<T> = std::result::Result<T, Rejection>;
pub type DBCon = Connection<PgConnectionManager<NoTls>>;
pub type DBPool = Pool<PgConnectionManager<NoTls>>;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();

    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    warp::serve(routes::bind(db_pool))
        .run(([127, 0, 0, 1], 8000))
        .await;
}
