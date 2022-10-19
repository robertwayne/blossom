use std::net::SocketAddr;

use nectar::{event::TelnetEvent, TelnetCodec};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use crate::error::{Error, ErrorType, Result};

/// Represents a players connection stream, as well as their write channel half.
/// The read half is owned by the server message loop. In general, the
/// connection should only be interacted with via the `send_message` and
/// `send_iac` methods.
pub struct Connection {
    addr: SocketAddr,
    frame: Framed<TcpStream, TelnetCodec>,
}

impl Connection {
    pub fn new(addr: SocketAddr, frame: Framed<TcpStream, TelnetCodec>) -> Self {
        Self { addr, frame }
    }

    /// Returns a mutable reference to the connection frame.
    pub fn frame_mut(&mut self) -> &mut Framed<TcpStream, TelnetCodec> {
        &mut self.frame
    }

    /// Sends a Telnet message to the client.
    pub async fn send_message(&mut self, string: &str) -> Result<()> {
        let event = TelnetEvent::Message(string.to_string());

        self.frame.send(event).await.map_err(|e| Error {
            kind: ErrorType::Internal,
            message: e.to_string(),
        })
    }

    /// Sends a Telnet IAC (Interpret As Command) message to the client and
    /// records their response.
    pub async fn send_iac(&mut self, command: TelnetEvent) -> Result<TelnetEvent> {
        self.frame.send(command).await?;

        let response = match self.frame.next().await {
            Some(Ok(response)) => response,
            Some(Err(e)) => {
                tracing::error!(%e, "Error sending IAC");
                return Err(Error {
                    kind: ErrorType::Internal,
                    message: e.to_string(),
                });
            }
            None => {
                tracing::error!("No response from IAC");
                return Err(Error {
                    kind: ErrorType::Internal,
                    message: "No response from IAC".to_string(),
                });
            }
        };

        Ok(response)
    }
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Telnet Protocol)", self.addr)
    }
}
