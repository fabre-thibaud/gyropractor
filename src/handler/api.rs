use crate::{model::alert::*, repo::alert::*, Clients, DBPool, Result};
use serde_derive::Deserialize;
use warp::{filters::ws::Message, reject, reply::json, Reply};

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    search: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
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

pub async fn create_alert(
    body: AlertRequest,
    db_pool: DBPool,
    clients: Clients,
) -> Result<impl Reply> {
    let alert = create(&db_pool, body).await;

    clients.read().await.iter().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text("Danger Will Robinson Danger !")));
        }
    });

    Ok(json(&AlertResponse::of(
        alert.map_err(|e| reject::custom(e))?,
    )))
}
