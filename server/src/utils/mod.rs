use anyhow::Result;
use std::time::Duration;
use tracing::log::LevelFilter;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub mod snowflake;
pub mod time;

pub async fn init_database(dsn: &String) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(dsn);

    opt.max_connections(500)
        .min_connections(10)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Info)
        .set_schema_search_path("public");

    let db = Database::connect(opt);

    Ok(db.await?)
}
