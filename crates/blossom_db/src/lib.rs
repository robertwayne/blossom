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

#[derive(Clone, Debug)]
pub struct Database(PgPool);

impl Database {
    /// Creates a new Postgres database connection pool using the configuration
    /// settings in your game/config.toml file. This can be cloned freely as it
    /// is wrapped in an Arc.
    pub async fn create() -> Result<Self, DatabaseError> {
        let config = Config::load().await?;
        let pool = PgPool::connect(&config.db_url()).await?;

        Ok(Database(pool))
    }
}
