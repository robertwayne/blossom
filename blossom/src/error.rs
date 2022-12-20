use nectar::error::TelnetError;

use crate::{config::ConfigError, database::DatabaseError, event::Event, scripting::ScriptError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorType {
    Database,
    Config,
    Script,
    Telnet,
    Io,
    Internal,
    Authentication,
    Channel,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorType,
    pub message: String,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<DatabaseError> for Error {
    fn from(err: DatabaseError) -> Self {
        Self {
            kind: ErrorType::Database,
            message: err.to_string(),
        }
    }
}
impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Self {
        Self {
            kind: ErrorType::Config,
            message: err.to_string(),
        }
    }
}

impl From<ScriptError> for Error {
    fn from(err: ScriptError) -> Self {
        Self {
            kind: ErrorType::Script,
            message: err.to_string(),
        }
    }
}

impl From<TelnetError> for Error {
    fn from(err: TelnetError) -> Self {
        Self {
            kind: ErrorType::Telnet,
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            kind: ErrorType::Io,
            message: err.to_string(),
        }
    }
}

impl From<flume::RecvError> for Error {
    fn from(err: flume::RecvError) -> Self {
        Self {
            kind: ErrorType::Channel,
            message: err.to_string(),
        }
    }
}

impl From<flume::SendError<Event>> for Error {
    fn from(err: flume::SendError<Event>) -> Self {
        Self {
            kind: ErrorType::Channel,
            message: err.to_string(),
        }
    }
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self {
            kind: ErrorType::Io,
            message: err.to_string(),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self {
            kind: ErrorType::Database,
            message: err.to_string(),
        }
    }
}

impl From<globwalk::GlobError> for Error {
    fn from(err: globwalk::GlobError) -> Self {
        Self {
            kind: ErrorType::Io,
            message: err.to_string(),
        }
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self {
            kind: ErrorType::Authentication,
            message: err.to_string(),
        }
    }
}
