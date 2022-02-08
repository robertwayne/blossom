use std::ops::Deref;

use blossom_config::Config;
use sqlx::postgres::PgPool;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    /// Creates a new Postgres connection pool.
    pub async fn create(config: &Config) -> Result<Self, DatabaseError> {
        let pool = PgPool::connect(&config.db_url()).await?;
        Ok(Self { pool })
    }
}

impl Deref for Database {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}
