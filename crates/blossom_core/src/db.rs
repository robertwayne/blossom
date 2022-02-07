use sqlx::postgres::PgPool;

use crate::{config::Config, error::Error};

/// Creates a new Postgres connection pool.
pub async fn create_pool(config: &Config) -> Result<PgPool, Error> {
    let pool = PgPool::connect(&config.db_url()).await?;
    Ok(pool)
}
