//! Database operations; squirrel code lives here.

use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pg_pool() -> Result<sqlx::Pool<sqlx::Postgres>> {
    let db_url = &std::env::var("DATABASE_URL")
        .expect("database url to be defined in the environment")[..];

    Ok(PgPoolOptions::new()
        // Postgres default max connections is 100, and we'll take 'em
        // https://www.postgresql.org/docs/current/runtime-config-connection.html
        .max_connections(80)
        .connect(db_url)
        .await?)
}
