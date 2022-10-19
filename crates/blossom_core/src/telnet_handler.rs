use std::net::SocketAddr;

use nectar::{event::TelnetEvent, TelnetCodec};
use flume::{unbounded, Sender};
use futures::StreamExt;
use sqlx::PgPool;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use crate::{
    auth::authenticate,
    connection::Connection,
    error::Result,
    event::{ClientEvent, Event, GameEvent},
    input::Input,
    response::Response,
};

pub async fn telnet_connection_loop(
    stream: TcpStream,
    addr: SocketAddr,
    pg: PgPool,
    tx_broker: Sender<Event>,
) -> Result<()> {
    let frame = Framed::new(stream, TelnetCodec::new(1024));
    let mut conn = Connection::new(addr, frame);

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

    // Retain player ID as a marker for their connection
    let id = player.id;

    // Create a channel for a connection
    let (tx, rx) = unbounded::<Event>();

    // Move the player off into the game thread
    tracing::trace!("{} authenticated: moving to game thread", player.name);
    tx_broker
        .send_async(Event::Client(
            player.id,
            ClientEvent::Connect(player, Some(tx)),
        ))
        .await?;

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
            result = conn.frame_mut().next() => match result {
                Some(Ok(msg)) => {
                    tracing::trace!("Received message: {:?}", msg);

                    match msg {
                        TelnetEvent::Message(msg) => {

                            if msg.trim().is_empty() {
                                tx_broker.send(Event::Client(id, ClientEvent::Ping))?;
                                continue;
                            }
                            tx_broker.send(Event::Client(id, ClientEvent::Command(Input::from(msg))))?;
                        }
                        _ => continue,
                    }


                },
                Some(Err(e)) => {
                    tracing::error!(%e, "Error reading from connection: {}", addr);
                    break;
                }
                None => {
                    tracing::info!("Telnet connection closed: {}", addr);
                    break;
                }
            }
        }
    }

    tx_broker.send(Event::Client(id, ClientEvent::Disconnect))?;
    conn.send_message("\nGoodbye!\n").await?;

    Ok(())
}
