use crate::{handler::*, DBPool, error};
use futures::{StreamExt, FutureExt};
use std::convert::Infallible;
use warp::{Filter, reject::Rejection};

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

fn api(db_pool: DBPool) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let alerts = warp::path("alerts");

    return warp::path("api")
        .and(alerts
            .and(warp::get())
            .and(warp::query())
            .and(with_db(db_pool.clone()))
            .and_then(api::list_alerts))
        .or(alerts
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(api::create_alert));
}

fn health(db_pool: DBPool) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    return warp::path!("health")
        .and(with_db(db_pool))
        .and_then(health::status);
}

fn ws_endpoint() -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    return warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();

                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        warn!("websocket error: {:?}", e);
                    }
                })
            })
        });
}

pub fn bind(db_pool: DBPool) -> impl Filter<Extract = (impl warp::Reply,), Error = Infallible> + Clone {
    let root = warp::path::end()
        .and(warp::get())
        .map(|| warp::http::StatusCode::OK);

    let routes = root
        .or(health(db_pool.clone()))
        .or(api(db_pool.clone()))
        .or(ws_endpoint())
        .with(warp::cors().allow_any_origin())
        .with(warp::log("gyropractor::access-logs"))
        .recover(error::handle_rejection);

    return routes;
}
