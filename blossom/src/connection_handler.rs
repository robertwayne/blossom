use std::net::SocketAddr;

use flume::{unbounded, Sender};
use nectar::TelnetCodec;
use sqlx::PgPool;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_util::codec::Framed;

use crate::{
    auth::authenticate,
    blossom_log,
    connection::{Connection, RawStream},
    error::Result,
    event::{ClientEvent, Event, GameEvent},
    input::Input,
    logging::{Action, Kind, Loggable},
    response::Response,
    server::StreamType,
};

pub async fn connection_loop(
    stream_type: StreamType,
    addr: SocketAddr,
    stream: TcpStream,
    pg: PgPool,
    tx_broker: Sender<Event>,
    tx_logger: Sender<Action>,
) -> Result<()> {
    let mut conn = match stream_type {
        StreamType::Telnet => {
            let frame = Framed::new(stream, TelnetCodec::new(1024));

            Connection::new(addr, RawStream::Telnet(frame), tx_logger.clone())
        }
        StreamType::WebSocket => {
            let config = WebSocketConfig {
                max_message_size: Some(1400),
                max_frame_size: Some(1400),
                ..WebSocketConfig::default()
            };
            let ws = tokio_tungstenite::accept_async_with_config(stream, Some(config))
                .await
                .expect("Error during the websocket handshake occurred");

            Connection::new(addr, RawStream::WebSocket(ws), tx_logger.clone())
        }
    };

    // Display the logo.
    let logo = tokio::fs::read_to_string("game/logo.txt").await;
    if let Ok(logo) = logo {
        conn.send_message(&logo).await?;
    }

    // Connection initialization. Log a player in (or create an account).
    let maybe_player = authenticate(&mut conn, pg.clone()).await?;
    if maybe_player.is_none() {
        tracing::info!("Authentication failed.");
        return Ok(());
    }

    // We know the player is valid because of the 'is_none' check above;
    // however, in order to keep rightward drift at a minimum in this already
    // long function, we break it early instead of utilizing if/else (+1
    // indent).
    let player = maybe_player.expect("This should never happen.");

    // Keep a copy of the player ID for event handling.
    let id = player.id;

    // Store a copy of the account ID on the connection for logging.
    conn.account_id = Some(player.account.id);

    // Create a channel for a connection
    let (tx, rx) = unbounded::<Event>();

    // Move the player off into the game thread
    tracing::trace!("{} authenticated: moving to game thread", player.name);
    tx_broker.send_async(Event::Client(player.id, ClientEvent::Connect(player, Some(tx)))).await?;

    loop {
        tokio::select! {
            // Handle messages received from the broker on this peers rx channel
            Ok(event) = rx.recv_async() => {
                tracing::trace!("Received event: {:?}", event);

                match event {
                    Event::Game(id, event_type) => match event_type {
                        GameEvent::Accepted(Response::Client(msg)) => {
                            // Send a command response to the player
                            conn.send_message(&msg).await?;
                            tx_broker.send(Event::Client(id, ClientEvent::Command(Input { command: "look".to_string(), args: Vec::new() })))?;
                        }
                        GameEvent::Command(response)  => {
                            match response {
                                Response::Client(msg) => {
                                    // Send a command response to the player
                                    conn.send_message(&msg).await?;

                                }
                                Response::Channel(_, msg) => {
                                    // Send a message to a channel
                                    conn.send_message(&msg).await?;
                                }
                                Response::Close => {
                                    break;
                                }
                                _ => continue,
                            }
                        }
                        GameEvent::Pong(response) => {
                            if let Response::Client(msg) = response {
                                conn.send_message(&msg).await?;
                            }
                        }
                        _ => continue,
                    }
                    _ => continue,
                }
            }
            // Handles messages received from the peer (via telnet)
            result = conn.try_next() => match result {
                Some(msg) => {
                    tracing::trace!("Received message: {:?}", msg);

                    if msg.trim().is_empty() {
                        tx_broker.send(Event::Client(id, ClientEvent::Ping))?;
                        continue;
                    }

                    blossom_log!(Kind::Message, msg.clone(), &conn);

                    tx_broker.send(Event::Client(id, ClientEvent::Command(Input::from(msg))))?;
                },
                None => {
                    tracing::info!("Connection closed: {}", addr);
                    break;
                }
            }
        }
    }

    blossom_log!(Kind::Leave, &conn);

    tx_broker.send(Event::Client(id, ClientEvent::Disconnect))?;
    conn.send_message("\nGoodbye!\n").await?;

    Ok(())
}
