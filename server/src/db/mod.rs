mod migrations;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub type PgPool = Pool<Postgres>;

pub async fn init_migrations(database_url: &str) -> Result<(), sqlx::error::Error> {
    {
        let pool = new_pg_pool(database_url).await?;
        migrations::exec_migrations(&pool).await?;
    }

    Ok(())
}

pub async fn new_pg_pool(database_url: &str) -> Result<PgPool, sqlx::error::Error> {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(500))
        .connect(database_url)
        .await
}
