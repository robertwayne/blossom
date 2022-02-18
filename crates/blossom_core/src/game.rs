use blossom_config::Config;
use flume::{Receiver, Sender};

use crate::{
    command::GameCommand,
    commands::{
        admin::{
            help::AdminHelp, player_info::PlayerInfo, shutdown::Shutdown,
            system_control::SystemsControl, world_info::WorldInfo,
        },
        afk::Afk,
        brief::Brief,
        help::Help,
        look::Look,
        ooc::GlobalChat,
        quit::Quit,
        say::Say,
        walk::Walk,
        who::Who,
    },
    event::Event,
    monster::MonsterTemplate,
    region::{AreaBuilder, RegionBuilder},
    room::RoomBuilder,
    scripting::{create_engine, get_game_objects},
    systems::{global_save::GlobalSave, spawner::Spawner, watcher::SystemWatcher},
    world::World,
};

/// Internal entry point for the game loop. Handles initialization of the world, adds systems,
/// commands, and loads up all script files before moving the world off to its own blocking thread.
pub struct Game;

impl Game {
    pub fn run(mut world: World, config: &Config, rx: Receiver<Event>, tx: Sender<Event>) {
        world.rx = rx;
        world.broker = tx;

        let engine = create_engine();

        world.add_system("watcher", SystemWatcher::new());
        world.add_system("global_save", GlobalSave::new(config.game.save_interval));
        world.add_system("spawner", Spawner::new(300));

        if config.game.default_commands {
            world.add_command(Afk::create(), Afk::run);
            world.add_command(Brief::create(), Brief::run);
            world.add_command(GlobalChat::create(), GlobalChat::run);
            world.add_command(Help::create(), Help::run);
            world.add_command(Look::create(), Look::run);
            world.add_command(Quit::create(), Quit::run);
            world.add_command(Say::create(), Say::run);
            world.add_command(Walk::create(), Walk::run);
            world.add_command(Who::create(), Who::run);
            world.add_command(Shutdown::create(), Shutdown::run);
            world.add_command(WorldInfo::create(), WorldInfo::run);
            world.add_command(SystemsControl::create(), SystemsControl::run);
            world.add_command(PlayerInfo::create(), PlayerInfo::run);
            world.add_command(AdminHelp::create(), AdminHelp::run);
        }

        // Game initialization for locations is done sequentially:
        // 1. We load all regions first, and place them into the location store.
        // 2. We load all the areas, and place them into the region they belong
        //    to. Area data files all contain a region by name, so we can just
        //    iterate through all the regions and fill them up.
        // 3. We load all the rooms, and place them into the area they belong
        //    to. Room data files all contain an area by name, so we can just
        //    iterate through all the areas and fill them up.
        // Now we have a can store a QuickMap of every region, area, and room
        // in the game for fast access and iteration.

        // Load all regions
        if let Ok(regions) = get_game_objects::<RegionBuilder>(&engine, "regions") {
            for builder in regions {
                let id = world.next_id();
                let r = builder.build(id);

                world.regions.push(r);
            }
        }

        // Load all areas
        if let Ok(areas) = get_game_objects::<AreaBuilder>(&engine, "areas") {
            for builder in areas {
                let id = world.next_id();
                let area = builder.build(id);

                world.areas.push(area);
            }
        }

        // Load all rooms
        if let Ok(rooms) = get_game_objects::<RoomBuilder>(&engine, "rooms") {
            for builder in rooms {
                let id = world.next_id();
                let room = builder.build(id);

                world.rooms.insert(room);
            }
        }

        // Load all monster templates
        if let Ok(monsters) = get_game_objects::<MonsterTemplate>(&engine, "monsters") {
            for template in monsters {
                world
                    .monsters
                    .insert_template(template.create_key(), template);
            }
        }

        tokio::task::spawn_blocking(move || world.start_loop());
    }
}
