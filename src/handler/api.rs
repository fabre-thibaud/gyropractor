use crate::{model::alert::*, DBPool, Result, repo::alert::*};
use serde_derive::Deserialize;
use warp::{reject, reply::json, Reply};

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    search: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>
}

pub async fn list_alerts(query: SearchQuery, db_pool: DBPool) -> Result<impl Reply> {
    debug!("{:?}", query);

    let alerts = fetch(&db_pool, query.search, query.page, query.page_size)
        .await
        .map_err(|e| reject::custom(e))?;

    Ok(json::<Vec<_>>(
        &alerts.into_iter().map(|t| AlertResponse::of(t)).collect(),
    ))
}

pub async fn create_alert(body: AlertRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&AlertResponse::of(
        create(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}
