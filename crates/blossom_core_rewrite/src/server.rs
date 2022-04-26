use std::collections::HashMap;

use bevy_app::App;
use bevy_ecs::system::{Res, ResMut};
use blossom_config::Config;
use blossom_db::Database;
use flume::{unbounded, Receiver, Sender};
use tokio::{
    net::TcpListener,
    runtime::{Handle, Runtime},
};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

use crate::{
    broker::Broker,
    event::{ClientEvent, Event},
    player::PlayerId,
    telnet_handler::telnet_connection_loop,
};

pub struct Game {
    pub rt: Runtime,
    pub app: App,
    pub rx: Receiver<Event>,
    pub tx: Sender<Event>,
}

impl Game {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        let rt = tokio::runtime::Builder::new_multi_thread()
            .thread_name("blossom-server")
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime");

        let db = rt.block_on(Database::create())?;
        let config = Config::load()?;

        // let (broker_tx, broker_rx) = unbounded::<Event>();
        let (game_tx, game_rx) = unbounded::<Event>();

        // let db_c1 = db.clone();

        // let _ = rt.spawn(async move {
        //     let _ = Broker::start(db_c1, broker_rx, game_tx);
        // });

        let listener = rt.block_on(TcpListener::bind(config.game_addr()))?;
        tracing::info!("Listening on {}", config.game_addr());

        let _db = db.clone();
        let connection_handle = async move {
            loop {
                let game_tx = game_tx.clone();
                let db = _db.clone();

                tokio::select! {
                    Ok((stream, addr)) = listener.accept() => {
                        let handle = Handle::current();

                        handle.spawn(async move {
                            tracing::info!("New connection from {}", addr);
                            if let Err(e) = telnet_connection_loop(stream, addr, db, game_tx).await {
                                tracing::error!(%e, "Error handling telnet connection");
                            }
                        });
                    }
                }
            }
        };

        rt.spawn(connection_handle);

        let peer_map: HashMap<PlayerId, Sender<Event>> = HashMap::new();

        let mut app = App::new();
        app.insert_resource(db)
            .insert_resource(config)
            .insert_resource(game_rx)
            .insert_resource(peer_map);
        app.set_runner(test_runner);
        app.add_system(test_system);

        app.run();

        Ok(())
    }
}

pub fn test_runner(mut app: App) {
    tracing::info!("Hi");
    loop {
        app.update();
    }
}

pub fn test_system(rx: Res<Receiver<Event>>, mut peers: ResMut<HashMap<PlayerId, Sender<Event>>>) {
    if let Ok(event) = rx.try_recv() {
        tracing::info!("Received event: {:?}", event);

        match event {
            Event::Client(id, e) => match e {
                // We know the tx will always be Some, because we ONLY send a Some value. However,
                // because the game should NOT know about the peer connection or have its send
                // channel, we need to make this an option so we can forward as a None.
                ClientEvent::Connect(player, tx) => {
                    tracing::info!("Client {} connected", id);
                    peers.insert(player.id, tx.expect("this should never happen"));
                }
                ClientEvent::Disconnect => {
                    tracing::info!("Client {} disconnected", id);
                    peers.remove(&id);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
