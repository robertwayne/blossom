use std::net::{IpAddr, SocketAddr};

use flume::Sender;
use futures::{SinkExt, StreamExt};
use nectar::{event::TelnetEvent, TelnetCodec};

use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use tokio_util::codec::Framed;

use crate::{
    error::{Error, ErrorType, Result},
    logging::{Action, Loggable},
};

pub enum RawStream {
    Telnet(Framed<TcpStream, TelnetCodec>),
    WebSocket(WebSocketStream<TcpStream>),
}

/// Represents a players connection stream, as well as their write channel half.
/// The read half is owned by the server message loop. In general, the
/// connection should only be interacted with via the `send_message` and
/// `send_iac` methods.
pub struct Connection {
    pub addr: SocketAddr,
    // frame: Framed<TcpStream, TelnetCodec>,
    stream: RawStream,
    // Should only be None before the player has authenticated.
    pub account_id: Option<i32>,
    tx_logger: Sender<Action>,
}

impl Connection {
    pub fn new(addr: SocketAddr, stream: RawStream, tx_logger: Sender<Action>) -> Self {
        Self { addr, stream, account_id: None, tx_logger }
    }

    pub fn ip(&self) -> IpAddr {
        self.addr.ip()
    }

    pub async fn try_next(&mut self) -> Option<String> {
        match &mut self.stream {
            RawStream::Telnet(frame) => {
                let msg = frame.next().await?;

                match msg {
                    Ok(TelnetEvent::Message(msg)) => Some(msg),
                    _ => None,
                }
            }
            RawStream::WebSocket(ws) => {
                let msg = ws.next().await;

                match msg {
                    Some(Ok(Message::Text(msg))) => Some(msg),
                    _ => None,
                }
            }
        }
    }

    /// Sends a Telnet or WebSocket message to the client.
    pub async fn send_message(&mut self, string: &str) -> Result<()> {
        match &mut self.stream {
            RawStream::Telnet(frame) => {
                let event = TelnetEvent::Message(string.to_string());

                frame
                    .send(event)
                    .await
                    .map_err(|e| Error { kind: ErrorType::Internal, message: e.to_string() })
            }
            RawStream::WebSocket(ws) => ws
                .send(Message::Text(string.to_string()))
                .await
                .map_err(|e| Error { kind: ErrorType::Internal, message: e.to_string() }),
        }
    }

    /// Sends a Telnet IAC (Interpret As Command) message to the client.
    pub async fn send_iac(&mut self, command: TelnetEvent) -> Result<()> {
        match &mut self.stream {
            RawStream::Telnet(frame) => {
                frame.send(command).await?;

                match frame.next().await {
                    Some(Ok(response)) => response,
                    Some(Err(e)) => {
                        tracing::error!(%e, "Error sending IAC");
                        return Err(Error { kind: ErrorType::Internal, message: e.to_string() });
                    }
                    None => {
                        tracing::error!("No response from IAC");
                        return Err(Error {
                            kind: ErrorType::Internal,
                            message: "No response from IAC".to_string(),
                        });
                    }
                };

                Ok(())
            }
            RawStream::WebSocket(_ws) => Ok(()),
        }
    }
}

impl Loggable for Connection {
    fn identifier(&self) -> (IpAddr, Option<i32>) {
        (self.addr.ip(), self.account_id)
    }

    fn get_logger(&self) -> Sender<Action> {
        self.tx_logger.clone()
    }
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Telnet Protocol)", self.addr)
    }
}
