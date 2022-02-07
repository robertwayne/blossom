use flume::unbounded;
use tokio::net::TcpListener;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

use crate::{
    broker::Broker, config::Config, db::create_pool, error::Result, event::Event, game::Game,
    telnet_handler::telnet_connection_loop, world::World,
};

/// Entry point of every Blossom game.
pub struct Server;

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    /// Entry point of every Blossom game. Starts the listening server via Tokio, spawns the game
    /// loop off in a separate thread, and then proccesses all incoming connections off to the main
    /// connection loop.
    #[tokio::main]
    pub async fn listen(&self, world: World) -> Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_span_events(FmtSpan::CLOSE)
            .init();

        // Loads the `config.toml` file in the /game directory. We also set the environment variable
        // for our database here, which means we MUST create the state AFTER these functions.
        let config = Config::load().await?;

        // Spawns a database pool (this can be cloned freely)
        let pg = create_pool(&config).await?;

        // Creates our connection listener
        let telnet_listener = TcpListener::bind(config.addr()).await?;

        // Create the broker and game channels for bidirectional communication
        let (tx_broker, rx_broker) = unbounded::<Event>();
        let (tx_game, rx_game) = unbounded::<Event>();

        // Starts the broker loop
        let _broker_handle = Broker::start(pg.clone(), rx_broker, tx_game).await?;

        // Create the world and starts the game loop on its own (blocking) thread
        Game::run(world, &config, rx_game, tx_broker.clone());

        tracing::info!("Server listening on {}", config.addr());

        loop {
            let pg = pg.clone();
            let tx_broker = tx_broker.clone();

            tokio::select! {
                Ok((stream, addr)) = telnet_listener.accept() => {
                    tokio::spawn(async move {
                        tracing::info!("New connection from {}", addr);
                        if let Err(e) = telnet_connection_loop(stream, addr, pg, tx_broker).await {
                            tracing::error!(%e, "Error handling telnet connection");
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
