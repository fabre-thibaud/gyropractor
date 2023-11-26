
mod db;
mod error;
mod handler;
mod model;
mod repo;
mod routes;
mod ws;

use std::{sync::Arc, collections::HashMap};

use dotenv::dotenv;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio::sync::RwLock;
use tokio_postgres::NoTls;
use warp::Rejection;
use ws::Client;

pub type Result<T> = std::result::Result<T, Rejection>;
pub type DBCon = Connection<PgConnectionManager<NoTls>>;
pub type DBPool = Pool<PgConnectionManager<NoTls>>;
pub type Clients = Arc<RwLock<HashMap<String, Client>>>;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    warp::serve(routes::bind(db_pool, clients))
        .run(([0, 0, 0, 0], 8000))
        .await;
}
