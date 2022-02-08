use std::ops::Deref;

use blossom_config::{Config, ConfigError};
use sqlx::postgres::PgPool;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("config error: {0}")]
    ConfigError(#[from] ConfigError),
}

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    /// Creates a new Postgres connection pool.
    pub async fn create() -> Result<Self, DatabaseError> {
        let config = Config::load().await?;
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
