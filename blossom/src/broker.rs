use std::sync::Arc;

use dashmap::DashMap;
use flume::{Receiver, Sender};
use sqlx::PgPool;

use crate::{
    error::{Error, ErrorType, Result},
    event::{ClientEvent, Event, GameEvent},
    player::PlayerId,
    response::Response,
};

pub struct Broker {
    pg: PgPool,
    tx_peers: DashMap<i32, Sender<Event>>,
    tx_game: Sender<Event>,
    rx: Receiver<Event>,
}

pub type BrokerHandle = Arc<Broker>;

impl Broker {
    pub async fn start(
        pg: PgPool,
        rx: Receiver<Event>,
        tx_game: Sender<Event>,
    ) -> Result<BrokerHandle> {
        let broker = Broker { pg, tx_peers: DashMap::new(), tx_game, rx };

        let handle = Arc::new(broker);

        // We create a new handle for the broker loop and move it in
        let loop_handle = Arc::clone(&handle);
        tokio::spawn(async move {
            let _ = loop_handle.broker_loop().await;
        });

        // Then we can return the original handle so the connection manager can
        // distribute clones to peers as they connect
        Ok(handle)
    }

    async fn broker_loop(&self) -> Result<()> {
        loop {
            tokio::select! {
                Ok(event) = self.rx.recv_async() => {
                    tracing::trace!("Received event: {:?}", event);

                    match event {
                        Event::Client(id, event_type) => self.handle_client_event(id, event_type).await?,
                        Event::Game(id, event_type) => self.handle_game_event(id, event_type).await?,
                    }
                }
            }
        }
    }

    /// Processes client events from the connection pool and passes them to the
    /// game thread.
    async fn handle_client_event(&self, id: i32, event_type: ClientEvent) -> Result<()> {
        tracing::trace!("Handling client event: {:?}", event_type);

        match event_type {
            ClientEvent::Connect(player, tx) => {
                // We know the tx will always be Some, because we ONLY send a
                // Some value. However, because the game should NOT know about
                // the peer connection or have its send channel, we need to make
                // this an option so we can forward as a None.
                self.tx_peers.insert(id, tx.expect("This should never happen."));
                self.to_game(id, ClientEvent::Connect(player, None)).await?;
            }
            ClientEvent::Command(msg) => {
                self.to_game(id, ClientEvent::Command(msg)).await?;
            }
            ClientEvent::Ping => {
                self.to_game(id, ClientEvent::Ping).await?;
            }
            ClientEvent::Disconnect => {
                self.to_game(id, ClientEvent::Disconnect).await?;
                self.tx_peers.remove(&id);
            }
        }
        Ok(())
    }

    /// Processes game events from the game thread and invokes the correct
    /// passing function.
    async fn handle_game_event(&self, id: PlayerId, event_type: GameEvent) -> Result<()> {
        tracing::trace!("Handling game event: {:?}", event_type);

        match event_type {
            GameEvent::Accepted(response) => {
                self.to_client(id, GameEvent::Accepted(response)).await?;
            }
            GameEvent::Command(response) => match &response {
                Response::Channel(here, _) => {
                    self.broadcast(here.clone(), GameEvent::Command(response)).await?;
                }
                _ => self.to_client(id, GameEvent::Command(response)).await?,
            },
            GameEvent::Pong(response) => {
                self.to_client(id, GameEvent::Pong(response)).await?;
            }
            GameEvent::Save(mut player) => {
                player.save(self.pg.clone()).await;
            }
            GameEvent::GlobalSave(players) => {
                for mut player in players {
                    player.save(self.pg.clone()).await;
                }
            }
        }
        Ok(())
    }

    /// Passes a client event to the game thread with their ID.
    async fn to_game(&self, id: PlayerId, event: ClientEvent) -> Result<()> {
        self.tx_game.send_async(Event::Client(id, event)).await?;

        Ok(())
    }

    /// Passes a game event to a specific client by ID.
    async fn to_client(&self, id: PlayerId, event: GameEvent) -> Result<()> {
        self.tx_peers
            .get(&id)
            .ok_or(Error {
                kind: ErrorType::Internal,
                message: "Peer does not exist.".to_string(),
            })?
            .send_async(Event::Game(id, event))
            .await?;

        Ok(())
    }

    /// Passes a game event to a group of clients represented as an array of
    /// IDs.
    async fn broadcast(&self, ids: Vec<PlayerId>, event: GameEvent) -> Result<()> {
        for id in ids {
            self.to_client(id, event.clone()).await?;
        }

        Ok(())
    }
}
