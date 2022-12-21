use std::net::IpAddr;

use flume::{Receiver, Sender};
use sqlx::{types::ipnetwork::IpNetwork, PgPool, Postgres, QueryBuilder};
use std::sync::Arc;

#[macro_export]
macro_rules! blossom_log {
    ($kind:expr, $target:expr) => {{
        let action = Action::new($kind, $target);
        let _ = $target.get_logger().send(action);
    }};
    ($kind:expr, $detail:expr, $target:expr) => {{
        let action = Action::with_detail($kind, $detail, $target);
        let _ = $target.get_logger().send(action);
    }};
}

pub type LoggerHandle = Arc<Logger>;

/// Structs which implement the `Loggable` trait are able to be identified by a
/// pair of IP and ID - generally a specific player / account or an unidentified
/// connection.
///
/// The ID is optional because it is not known until the player has
/// authenticated, and may be used for implementing `Loggable` on system events.
///
/// A system event should return an IP of "127.0.0.1".
pub trait Loggable {
    fn identifier(&self) -> (IpAddr, Option<i32>);
    fn get_logger(&self) -> Sender<Action>;
}

/// Represents a logger containing a channel for receiving `Action` events.
pub struct Logger {
    rx: Receiver<Action>,
}

impl Logger {
    /// Starts the global logging loop. The logger collects actions and batch
    /// processes them on a timed interval to reduce load on the database.
    pub async fn start(pg: PgPool, rx: Receiver<Action>) -> crate::error::Result<LoggerHandle> {
        let logger = Logger { rx };
        let handle = Arc::new(logger);
        let loop_handle = Arc::clone(&handle);

        tokio::spawn(async move {
            let mut queue: Vec<Action> = Vec::new();
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

            loop {
                tokio::select! {
                    Ok(action) = loop_handle.rx.recv_async() => {
                        queue.push(action);
                    }
                    _ = interval.tick() => {
                        if queue.is_empty() {
                            continue;
                        }

                        if let Err(e) = Self::process_queue(&mut queue, &pg).await {
                            tracing::error!("Error logging actions: {}", e);
                        }
                    }
                }
            }
        });

        Ok(handle)
    }

    /// Creates a query builder to batch insert actions into the database.
    async fn process_queue(queue: &mut Vec<Action>, pg: &PgPool) -> Result<(), sqlx::Error> {
        tracing::debug!("Logging {} actions", queue.len());

        let mut query_builder: sqlx::QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO action_logs (account_id, ip_address, kind, detail, created_on)",
        );

        let queue = queue.drain(..);
        query_builder.push_values(queue.take(65535 / 5), |mut b, action| {
            b.push_bind(action.id)
                .push_bind(action.addr)
                .push_bind(action.kind.to_string())
                .push_bind(action.detail)
                .push_bind(action.created_on);
        });
        let query = query_builder.build();
        query.execute(pg).await?;

        Ok(())
    }
}

/// Represents a type of action that can be logged.
pub enum Kind {
    CreateAccount,
    Join,
    FailedJoin,
    Leave,
    Message,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::CreateAccount => write!(f, "create_account"),
            Kind::Join => write!(f, "join"),
            Kind::FailedJoin => write!(f, "failed_join"),
            Kind::Leave => write!(f, "leave"),
            Kind::Message => write!(f, "message"),
        }
    }
}

/// Represents a "complete" action to be logged into the database at a later
/// time.
pub struct Action {
    id: Option<i32>,
    addr: IpNetwork,
    kind: Kind,
    detail: Option<String>,
    created_on: time::OffsetDateTime,
}

impl Action {
    /// Create a basic action that has no extraneous details.
    pub fn new(kind: Kind, target: &impl Loggable) -> Self {
        let (ip, id) = target.identifier();

        Self {
            id,
            addr: IpNetwork::from(ip),
            kind,
            detail: None,
            created_on: time::OffsetDateTime::now_utc(),
        }
    }

    /// Create an action with a detail string.
    pub fn with_detail(kind: Kind, detail: String, target: &impl Loggable) -> Self {
        let (ip, id) = target.identifier();

        Self {
            id,
            addr: IpNetwork::from(ip),
            kind,
            detail: Some(detail),
            created_on: time::OffsetDateTime::now_utc(),
        }
    }
}
