use sqlx::{types::ipnetwork::IpNetwork, PgPool};

use crate::connection::Connection;

/// Represents a loggable action that a player or IP has taken.
#[derive(Debug)]
pub enum Action {
    CreateAccount,
    Join,
    FailedJoin,
    Leave,
    Message(String),
}

impl Action {
    /// Returns a string representation of the enum. Used primarily for
    /// simplifying analysis of logs.
    pub fn kind(&self) -> &str {
        match self {
            Action::CreateAccount => "create_account",
            Action::Join => "join",
            Action::FailedJoin => "failed_join",
            Action::Leave => "leave",
            Action::Message(_) => "message",
        }
    }

    /// Returns extraneous data attached to the message kind, typically the
    /// command a user sent to the server.
    pub fn details(&self) -> &str {
        match self {
            Action::Message(msg) => msg,
            _ => "",
        }
    }
}

/// Logs an action to the database.
///
/// Actions taken without a logged in account will be represented with an
/// account_id of -1. This should only happen on `FailedJoin` actions, as all
/// other actions are tied to accounts.
pub async fn log(
    action: Action,
    account_id: Option<i32>,
    conn: &Connection,
    pg: &PgPool,
) -> Result<(), sqlx::Error> {
    let id = match account_id {
        Some(id) => id,
        None => -1,
    };

    let _ = sqlx::query!(
        "INSERT INTO action_logs (account_id, action, ip_address, details) VALUES ($1, $2, $3, $4)",
        id,
        action.kind(),
        IpNetwork::from(conn.ip()),
        action.details()
    )
    .execute(&*pg)
    .await?;

    Ok(())
}
