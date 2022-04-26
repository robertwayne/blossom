use bevy_app::App;
use blossom_config::Config;
use flume::{Receiver, Sender};
use tracing::debug;

use crate::{
    command::GameCommand,
    commands::help::Help,
    event::{ClientEvent, Event},
    scripting::{create_engine, get_game_objects},
    systems::process_commands::process_commands,
    world::{Connection, World},
};

/// Internal entry point for the game loop. Handles initialization of the world, adds systems,
/// commands, and loads up all script files before moving the world off to its own blocking thread.
pub struct Game;

impl Game {
    pub fn run(mut world: World, config: &Config, rx: Receiver<Event>, tx: Sender<Event>) {
        let engine = create_engine();

        // world.add_system("watcher", SystemWatcher::new());
        // world.add_system("global_save", GlobalSave::new(config.game.save_interval));

        if config.game.default_commands {
            // world.add_command(Afk::create(), Afk::run);
            // world.add_command(Brief::create(), Brief::run);
            // world.add_command(GlobalChat::create(), GlobalChat::run);
            world.add_command(Help::create(), Help::run);
            // world.add_command(Look::create(), Look::run);
            // world.add_command(Quit::create(), Quit::run);
            // world.add_command(Say::create(), Say::run);
            // world.add_command(Walk::create(), Walk::run);
            // world.add_command(Who::create(), Who::run);
            // world.add_command(Shutdown::create(), Shutdown::run);
            // world.add_command(WorldInfo::create(), WorldInfo::run);
            // world.add_command(SystemsControl::create(), SystemsControl::run);
            // world.add_command(PlayerInfo::create(), PlayerInfo::run);
            // world.add_command(AdminHelp::create(), AdminHelp::run);
        }
        world
            .app
            .insert_resource(Connection { rx, tx })
            .set_runner(test_runner);

        world.app.run();
        // tokio::task::spawn_blocking(move || world.start_loop());
    }
}

fn test_runner(mut app: App) {
    debug!("Processing commands");
    while let Ok(Event::Client(id, event)) = app.world.resource::<Connection>().rx.try_recv() {
        tracing::debug!("Received event: {:?}", event);
        match event {
            ClientEvent::Connect(player, _) => {
                debug!("Player connected: {:?}", player);
            }
            _ => {}
        }
    }

    app.update();
}
