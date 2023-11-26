
mod db;
mod error;
mod handler;
mod model;
mod repo;
mod routes;
mod ws;

use std::{sync::Arc, collections::HashMap, net::IpAddr};

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
        .run(get_bind_addr())
        .await;
}

fn get_bind_addr() -> (IpAddr, u16)
{
    let bind_addr = std::env::var("GYRO_LISTEN_ADDR").expect("GYRO_LISTEN_ADDR env var is not set");
    let bind_port = std::env::var("GYRO_LISTEN_PORT").expect("GYRO_LISTEN_PORT env var is not set");

    (
        bind_addr.parse::<IpAddr>().expect("GYRO_LISTEN_ADDR is not a valid IP address"),
        bind_port.parse::<u16>().expect("GYRO_LISTEN_PORT is not a valid port")
    )
}
