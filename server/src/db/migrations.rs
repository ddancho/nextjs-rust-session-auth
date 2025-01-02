use super::PgPool;

pub async fn exec_migrations(pool: &PgPool) -> Result<(), sqlx::error::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;

    Ok(())
}
