use flume::{RecvError, SendError};
use thiserror::Error;

use crate::{event::Event, player::PlayerId, scripting};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("environment variable {0} not set")]
    EnvVarNotSet(#[from] std::env::VarError),
    #[error("script error: {0}")]
    ScriptError(#[from] scripting::BlossomScriptError),
    #[error("invalid configuration: {0}")]
    ConfigError(#[from] toml::de::Error),
    #[error("serialize error: {0}")]
    SerializeError(#[from] toml::ser::Error),
    #[error("ioerror: {0}")]
    IOError(#[from] std::io::Error),
    #[error("auth error: {0}")]
    AuthError(String),
    #[error("protocol error: {0}")]
    ProtocolError(String),
    #[error("telnet error: {0}")]
    TelnetError(String),
    #[error("broker send error: {0}")]
    BrokerWriteError(#[from] SendError<Event>),
    #[error("broker read error: {0}")]
    BrokerReadError(#[from] RecvError),
    #[error("peer does not exist: {0}")]
    PeerDoesNotExist(PlayerId),
    #[error("entity not found: {0}")]
    EntityNotFound(String),
    #[error("invalid glob pattern: {0}")]
    BadPattern(#[from] globwalk::GlobError),
}
