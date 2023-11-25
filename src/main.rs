
mod db;
mod error;
mod handler;
mod model;
mod repo;

use dotenv::dotenv;
use futures::{StreamExt, FutureExt};
use handler::*;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

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

    let root_route = warp::path::end()
        .and(warp::get())
        .map(|| warp::http::StatusCode::OK);

    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(health::status);

    let ws = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let api = warp::path("api");
    let alerts = warp::path("alerts");

    let alert_routes = alerts
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db_pool.clone()))
        .and_then(api::list_alerts)
        .or(alerts
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(api::create_alert));

    let routes = root_route
        .or(health_route)
        .or(api.and(alert_routes))
        .or(ws)
        .with(warp::cors().allow_any_origin())
        .with(warp::log("gyropractor::access-logs"))
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
