use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid configuration: {0}")]
    ConfigError(#[from] toml::de::Error),
    #[error("serialize error: {0}")]
    SerializeError(#[from] toml::ser::Error),
    #[error("parse error: {0}")]
    ParseError(#[from] std::io::Error),
}

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub game: GameSettings,
    pub web: WebSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Serialize)]
pub struct GameSettings {
    pub host: String,
    pub port: u16,
    pub tick_rate: u64,
    pub save_interval: u64,
    pub default_commands: bool,
}

#[derive(Deserialize, Serialize)]
pub struct WebSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Serialize)]
pub struct DatabaseSettings {
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_host: String,
    pub db_port: i16,
}

impl Config {
    /// Attempts to read in a `config.toml` file from the root directory of your game. This file is
    /// required for the game to run, so this will panic if there are ANY errors.
    pub async fn load() -> Result<Self, ConfigError> {
        let dir_exists = std::fs::metadata("game").is_ok();

        if !dir_exists {
            tracing::info!("No game directory found. Creating one now at `/game`.");

            std::fs::create_dir("game")?;
        }

        let exists = std::fs::metadata("game/config.toml").is_ok();

        if !exists {
            tracing::info!("No configuration file found. Creating one now at `/game/config.toml`.");
            tracing::info!("You may need to change the default values in order to run your game.");

            let config = &toml::to_string(&Config::default())?;

            std::fs::write("game/config.toml", config)?;
        }

        let path = std::fs::read_to_string("game/config.toml")?;
        let config: Config = toml::from_str(&path)?;

        if config.game.tick_rate > 20 {
            tracing::warn!("Server tick_rate is very high. This may cause excessive CPU usage and/or performance issues.");
        }

        if config.game.save_interval < 30 {
            tracing::warn!("Server save_interval is very fast. This may cause excessive database writes and/or performance issues.");
        }

        Ok(config)
    }

    /// Returns the listening server address as HOST:PORT.
    pub fn game_addr(&self) -> String {
        format!("{}:{}", self.game.host, self.game.port)
    }

    /// Returns the web server address as HOST:PORT.
    pub fn web_addr(&self) -> String {
        format!("{}:{}", self.web.host, self.web.port)
    }

    /// Returns the database connection string.
    pub fn db_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.database.db_user,
            self.database.db_pass,
            self.database.db_host,
            self.database.db_port,
            self.database.db_name
        )
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            host: "127.0.0.1".to_string(),
            port: 5000,
            tick_rate: 20,
            save_interval: 300,
            default_commands: true,
        }
    }
}

impl Default for WebSettings {
    fn default() -> Self {
        WebSettings {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        DatabaseSettings {
            db_name: "postgres".to_string(),
            db_user: "postgres".to_string(),
            db_pass: "".to_string(),
            db_host: "postgres".to_string(),
            db_port: 5432,
        }
    }
}