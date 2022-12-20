use flume::unbounded;
use tokio::net::TcpListener;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

use crate::{
    broker::Broker, config::Config, connection_handler::connection_loop, database::Database,
    error::Result, event::Event, game::Game, world::World,
};

pub enum StreamType {
    Telnet,
    WebSocket,
}

/// Entry point of every Blossom game.
pub struct Server;

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    /// Entry point of every Blossom game. Starts the listening server via
    /// Tokio, spawns the game loop off in a separate thread, and then
    /// proccesses all incoming connections off to the main connection loop.
    #[tokio::main]
    pub async fn listen(&self, world: World) -> Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_span_events(FmtSpan::CLOSE)
            .init();

        // Spawns a database pool (this can be cloned freely)
        let db = Database::create().await?;

        // Loads the `config.toml` file in the /game directory. We also set the
        // environment variable for our database here, which means we MUST
        // create the state AFTER these functions.
        let config = Config::load().await?;

        // Creates our connection listener
        let telnet_listener = TcpListener::bind(config.telnet_addr()).await?;
        let websocket_listener = TcpListener::bind(config.websocket_addr()).await?;

        if config.web.enabled {
            let pg = db.clone();
            tokio::spawn(async move {
                crate::web::listen(pg)
                    .await
                    .expect("Failed to bind to address");
            });
        }

        // Create the broker and game channels for bidirectional communication
        let (tx_broker, rx_broker) = unbounded::<Event>();
        let (tx_game, rx_game) = unbounded::<Event>();

        // Starts the broker loop
        let _broker_handle = Broker::start(db.clone(), rx_broker, tx_game).await?;

        // Create the world and starts the game loop on its own (blocking)
        // thread
        Game::run(world, &config, rx_game, tx_broker.clone());

        tracing::info!(
            "Server listening on {} (Telnet) and {} (WebSocket)",
            config.telnet_addr(),
            config.websocket_addr()
        );

        loop {
            let pg = db.clone();
            let tx_broker = tx_broker.clone();

            tokio::select! {
                Ok((stream, addr)) = telnet_listener.accept() => {
                    tokio::spawn(async move {
                        tracing::info!("New connection from {}", addr);

                        if let Err(e) = connection_loop(StreamType::Telnet, addr, stream, pg, tx_broker).await {
                            tracing::error!(%e, "Failed to establish Telnet stream");
                        }
                    });
                }
                Ok((stream, addr)) = websocket_listener.accept() => {
                    tokio::spawn(async move {
                        tracing::info!("New connection from {}", addr);

                        if let Err(e) = connection_loop(StreamType::WebSocket, addr, stream, pg, tx_broker).await {
                            tracing::error!(%e, "Failed to establish WebSocket stream");
                        }
                    });
                }
            }
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}