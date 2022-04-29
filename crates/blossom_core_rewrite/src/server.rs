use std::collections::HashMap;

use bevy_app::{App, CoreStage};
use bevy_ecs::prelude::*;

use bevy_ecs::schedule::{Schedule, SystemStage};
use blossom_config::Config;
use blossom_db::Database;
use flume::{unbounded, Sender};

use once_cell::sync::{Lazy, OnceCell};
use parking_lot::Mutex;
use tokio::{
    net::TcpListener,
    runtime::{Handle, Runtime},
};
use tracing_subscriber::EnvFilter;

use crate::{
    command::{CommandHandle, GameCommand},
    commands::help::Help,
    event::{Event, GameEvent},
    input_system::input_system,
    player::PlayerId,
    response::Response,
    runner::runner,
    stores::system_store::SystemStore,
    system,
    system::{SystemHandle, SystemReadOnly, SystemReadOnlyHandle, SystemStatus},
    systems::{execution_timer::ExecutionTimer, watcher::SystemWatcher},
    telnet_handler::telnet_connection_loop,
    timer::Timer,
};

pub type GameCommandMap = HashMap<String, CommandHandle>;
pub type Peers = HashMap<PlayerId, Sender<Event>>;

pub static DYNAMIC_SYSTEMS: Lazy<Mutex<SystemStore>> = Lazy::new(|| Mutex::new(SystemStore::new()));

pub struct Game {}

impl Game {
    pub fn add_system(name: &'static str, system: impl system::System + 'static) {
        DYNAMIC_SYSTEMS
            .lock()
            .write
            .push(SystemHandle::new(name, Box::new(system)));
    }

    pub fn add_system_readonly(name: &'static str, system: impl SystemReadOnly + 'static) {
        DYNAMIC_SYSTEMS
            .lock()
            .readonly
            .push(SystemReadOnlyHandle::new(name, Box::new(system)));
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_max_level(tracing::Level::DEBUG)
            .init();

        let rt = tokio::runtime::Builder::new_multi_thread()
            .thread_name("blossom-server")
            .enable_all()
            .build()
            .expect("failed to create a tokio runtime");

        let db = rt.block_on(Database::create())?;
        let config = Config::load()?;

        let (game_tx, game_rx) = unbounded::<Event>();

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
                            tracing::debug!("New connection from {}", addr);

                            if let Err(e) = telnet_connection_loop(stream, addr, db, game_tx).await {
                                tracing::error!(%e, "error handling telnet connection");
                            }
                        });
                    }
                }
            }
        };

        rt.spawn(connection_handle);

        let peer_map: HashMap<PlayerId, Sender<Event>> = HashMap::new();
        let mut commands = HashMap::new();

        // Add default dynamic systems
        // let stores = DYNAMIC_SYSTEMS.lock();
        // stores
        //     .write
        //     .push(SystemHandle::new("watcher", Box::new(SystemWatcher::new())));

        let help = Help::create();
        commands.insert(
            help.name.clone(),
            CommandHandle {
                inner: help,
                func: Box::new(Help::run),
            },
        );

        let mut app = App::new();
        app.insert_resource(db)
            .insert_resource(config)
            .insert_resource(game_rx)
            .insert_resource(peer_map)
            .insert_resource(commands)
            .insert_resource(Timer::new());
        app.set_runner(runner);
        app.add_system(input_system);
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            dynamic_system_handler.exclusive_system().at_end(),
        );

        app.run();

        Ok(())
    }
}

// pub fn dynamic_write_system_handler(world: &mut World) {
//     let cell = world.cell();
//     let mut stores = cell.get_resource_mut::<SystemStore>().unwrap();

//     for system in stores.write.iter_mut() {
//         if let SystemStatus::Running = system.status {
//             system.inner.update(world);
//         }
//     }
// }

pub fn dynamic_system_handler(world: &mut World) {
    // let stores = world.get_resource::<SystemStore>().unwrap();
    let mut stores = DYNAMIC_SYSTEMS.lock();

    for system in stores.readonly.iter() {
        if let SystemStatus::Running = system.status {
            system.inner.update(world);
        }
    }

    for system in stores.write.iter_mut() {
        if let SystemStatus::Running = system.status {
            system.inner.update(world);
        }
    }
}

/// Helper function for sending a generic GameEvent to a peer. This only exists
/// for simplfying the API.
#[inline(always)]
pub fn send_event(id: PlayerId, tx: &Sender<Event>, event: GameEvent) {
    let _ = tx.send(Event::Game(id, event));
}

/// Helper function for sending a GameEvent::Command to a peer. It only exists
/// for simplfying the API, as this is the most common event type to send.
#[inline(always)]
pub fn send_command(id: PlayerId, tx: &Sender<Event>, response: Response) {
    send_event(id, tx, GameEvent::Command(response));
}
