use crate::{error, error::Error::*, DBCon, DBPool};
use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls};

pub type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const INIT_SQL: &str = "./res/db.sql";

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
    Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let postgres_host: String =
        std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST env var is not set");
    let postgres_port: String =
        std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT env var is not set");
    let postgres_user: String =
        std::env::var("POSTGRES_USER").expect("POSTGRES_USER env var is not set");
    let postgres_pass: String =
        std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD is not set");
    let postgres_database: String =
        std::env::var("POSTGRES_DATABASE").expect("POSTGRES_DATABASE is not set");

    let config_uri: String = format!(
        "postgres://{}:{}@{}:{}/{}",
        postgres_user, postgres_pass, postgres_host, postgres_port, postgres_database
    );

    debug!("connecting to postgres URI {}", config_uri);

    let config = Config::from_str(&config_uri)?;
    let manager = PgConnectionManager::new(config, NoTls);

    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}
