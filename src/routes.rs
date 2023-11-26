use crate::{error, handler::*, Clients, DBPool};
use std::convert::Infallible;
use warp::{reject::Rejection, Filter};

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn api(
    db_pool: DBPool,
    clients: Clients,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    debug!("setting up api routes");

    let alerts = warp::path("alerts");

    warp::path("api").and(
        alerts
            .and(warp::get())
            .and(warp::query())
            .and(with_db(db_pool.clone()))
            .and_then(api::list_alerts)
            .or(alerts
                .and(warp::post())
                .and(warp::body::json())
                .and(with_db(db_pool.clone()))
                .and(with_clients(clients.clone()))
                .and_then(api::create_alert)),
    )
}

fn health(
    db_pool: DBPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    debug!("setting up health routes");

    warp::path!("health")
        .and(with_db(db_pool))
        .and_then(health::status)
}

fn register(
    clients: Clients,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    debug!("setting up register routes");

    let register = warp::path("register");

    warp::path("api").and(
        register
            .and(warp::post())
            .and(warp::body::json())
            .and(with_clients(clients.clone()))
            .and_then(ws::register)
            .or(register
                .and(warp::delete())
                .and(warp::path::param())
                .and(with_clients(clients.clone()))
                .and_then(ws::unregister)),
    )
}

fn ws_endpoint(
    clients: Clients,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    debug!("setting up websocket endpoing");

    warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(ws::handler)
}

pub fn bind(
    db_pool: DBPool,
    clients: Clients,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Infallible> + Clone {
    debug!("setting up routes");

    let root = warp::path::end()
        .and(warp::get())
        .map(|| warp::http::StatusCode::OK);

    let routes = root
        .or(health(db_pool.clone()))
        .or(api(db_pool.clone(), clients.clone()))
        .or(register(clients.clone()))
        .or(ws_endpoint(clients.clone()))
        .with(warp::cors().allow_any_origin())
        .with(warp::log("gyropractor::access-logs"))
        .recover(error::handle_rejection);

    return routes;
}
