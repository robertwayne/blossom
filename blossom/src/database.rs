use sqlx::postgres::PgPool;

use crate::config::{Config, ConfigError};

#[derive(Debug)]
pub struct Database;

impl Database {
    /// Creates a new Postgres database connection pool using the configuration
    /// settings in your game/config.toml file. This can be cloned freely as it
    /// is wrapped in an Arc.
    pub async fn create() -> Result<PgPool, DatabaseError> {
        let config = Config::load().await?;
        let pool = PgPool::connect(&config.db_url()).await?;

        Ok(pool)
    }
}

#[derive(Debug)]
pub enum DatabaseErrorType {
    Config,
    Connection,
}

#[derive(Debug)]
pub struct DatabaseError {
    pub kind: DatabaseErrorType,
    pub message: String,
}

impl std::error::Error for DatabaseError {}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<ConfigError> for DatabaseError {
    fn from(err: ConfigError) -> Self {
        Self { kind: DatabaseErrorType::Config, message: err.to_string() }
    }
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        Self { kind: DatabaseErrorType::Connection, message: err.to_string() }
    }
}
