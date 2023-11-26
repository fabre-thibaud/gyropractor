use crate::{db::get_db_con, db::Result, error::Error::*, model::alert::*, DBPool};
use chrono::prelude::*;
use mobc_postgres::tokio_postgres::Row;

const TABLE: &str = "alerts";
const SELECT_FIELDS: &str = "id, name, component, checked, created_at, checked_at";

pub async fn fetch(
    db_pool: &DBPool,
    search: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<Vec<Alert>> {
    let con = get_db_con(db_pool).await?;

    let where_clause = match search {
        Some(_) => "WHERE name LIKE $1",
        None => "",
    };

    let limit: u32 = match page_size {
        Some(page_size) => std::cmp::min(page_size, 10000),
        None => 10,
    };

    let offset: u32 = match page {
        Some(page) => limit * (std::cmp::max(page, 1) - 1),
        None => 0,
    };

    let query = format!(
        "SELECT {} FROM {} {} ORDER BY created_at DESC LIMIT {} OFFSET {}",
        SELECT_FIELDS, TABLE, where_clause, limit, offset
    );

    debug!("{}", query);

    let q = match search {
        Some(v) => con.query(query.as_str(), &[&v]).await,
        None => con.query(query.as_str(), &[]).await,
    };

    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_alert(&r)).collect())
}

pub async fn create(db_pool: &DBPool, body: AlertRequest) -> Result<Alert> {
    debug!("{:?}", body);

    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (name, component) VALUES ($1, $2) RETURNING *",
        TABLE
    );
    let row = con
        .query_one(query.as_str(), &[&body.name, &body.component])
        .await
        .map_err(DBQueryError)?;

    Ok(row_to_alert(&row))
}

fn row_to_alert(row: &Row) -> Alert {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let component: String = row.get(2);
    let checked: bool = row.get(3);
    let created_at: DateTime<Utc> = row.get(4);
    let checked_at: Option<DateTime<Utc>> = row.get(5);

    Alert {
        id,
        name,
        component,
        created_at,
        checked,
        checked_at,
    }
}
