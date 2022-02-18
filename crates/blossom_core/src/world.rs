use std::{
    collections::HashMap,
    thread::sleep,
    time::{Duration, Instant},
};

use flume::{Receiver, Sender};
use iridescent::{
    constants::{CYAN, GREEN, YELLOW},
    Styled,
};

use crate::{
    command::{Command, CommandHandle},
    context::Context,
    entity::EntityId,
    error::{Error, ErrorType, Result},
    event::{ClientEvent, Event, GameEvent},
    monster::Monster,
    player::{Player, PlayerId},
    prompt::Prompt,
    quickmap::QuickMap,
    region::{Area, Region},
    response::Response,
    room::Room,
    stores::{monster_store::MonsterStore, system_store::SystemStore},
    system::{System, SystemHandle, SystemReadOnly, SystemReadOnlyHandle, SystemStatus},
    timer::Timer,
    vec3::Vec3,
};

/// Stateful representation of the game world, containing references to all game entities,
/// channels needed for between the broker and the game loop, and all of the systems and
/// commands added at startup.
pub struct World {
    pub rx: Receiver<Event>,
    pub broker: Sender<Event>,
    pub players: QuickMap<PlayerId, Player>,
    pub regions: Vec<Region>,
    pub areas: Vec<Area>,
    pub rooms: QuickMap<Vec3, Room>,
    pub monsters: MonsterStore,
    pub timer: Timer,
    pub systems: SystemStore,
    pub command_map: HashMap<String, usize>,
    pub commands: Vec<CommandHandle>,
    pub spawned_entities: u32,
    pub active_entities: u32,
}

impl World {
    pub fn new() -> Self {
        let (tx, rx) = flume::bounded(0);

        World {
            rx,
            broker: tx,
            players: QuickMap::new(),
            regions: Vec::new(),
            areas: Vec::new(),
            rooms: QuickMap::new(),
            monsters: MonsterStore::new(),
            timer: Timer::new(),
            systems: SystemStore::new(),
            command_map: HashMap::new(),
            commands: Vec::new(),
            spawned_entities: 0,
            active_entities: 0,
        }
    }

    pub fn start_loop(&mut self) {
        loop {
            let start = Instant::now();

            // Process the command queue
            self.process_commands();

            // Run game-specific, readonly systems
            for system in self.systems.readonly.iter() {
                if let SystemStatus::Running = system.status {
                    system.inner.update(self);
                }
            }

            // Run game-specific, writeable systems
            let mut systems = std::mem::take(&mut self.systems.write);
            for system in systems.iter_mut() {
                if let SystemStatus::Running = system.status {
                    system.inner.update(self);
                }
            }
            self.systems.write = systems;

            // Internal system for tracking execution time of game ticks.
            self.systems.execution_timer.update(start);

            self.tick()
        }
    }

    /// Loops through commands received from the broker and processes them. Note that this is not
    /// ONLY for game-specific commands, but all peer-sent messages that are valid and parsed as
    /// an Input struct. This includes connecting and disconnecting.
    fn process_commands(&mut self) {
        while let Ok(Event::Client(id, event)) = self.rx.try_recv() {
            tracing::trace!("Processing command: {:?}", event);

            match event {
                ClientEvent::Connect(mut player, _) => {
                    self.send_event(
                        id,
                        GameEvent::Accepted(Response::Client(format!(
                            "\n{}{}{}\n",
                            "Welcome back, ".foreground(YELLOW),
                            player.name.foreground(CYAN),
                            "!".foreground(YELLOW),
                        ))),
                    );

                    player._entityid = self.next_id();

                    self.players.insert(player);
                    self.timer.last_action = Instant::now()
                        .duration_since(self.timer.start_time)
                        .as_secs();
                }
                ClientEvent::Disconnect => {
                    if let Ok(player) = self.get_player(id) {
                        // Sends a Save event to the broker which will handle the actual database
                        // update. We only send this if the player is marked for saving; same
                        // as global save.
                        if player.dirty {
                            self.send_event(id, GameEvent::Save(player.clone()));
                        }

                        self.active_entities -= 1;

                        // Remove the player from the world
                        self.players.remove(&id);
                        self.timer.last_action = Instant::now()
                            .duration_since(self.timer.start_time)
                            .as_secs();
                    }
                }
                ClientEvent::Ping => self.send_prompt(id),
                ClientEvent::Command(tokens) => {
                    let result = match self.command_map.get(&tokens.command) {
                        Some(i) => {
                            let mut commands = std::mem::take(&mut self.commands);
                            let func = &mut commands[*i].func;
                            let result = (func)(Context::new(id, tokens, self));
                            self.commands = commands;

                            result
                        }
                        None => self.unknown(id),
                    };

                    if let Ok(response) = result {
                        self.send_command(id, response);
                    }

                    self.send_prompt(id);

                    self.timer.last_action = Instant::now()
                        .duration_since(self.timer.start_time)
                        .as_secs();
                }
            }
        }
    }

    /// Sends a `GameEvent` to the broker.
    pub fn send_event(&self, id: PlayerId, event: GameEvent) {
        let _ = self.broker.send(Event::Game(id, event));
    }

    /// Sends a `GameEvent::Command` to the broker. This is just a wrapper around `to_broker` to
    /// simplify the Command API, as it is the most common event type.
    pub fn send_command(&self, id: PlayerId, response: Response) {
        self.send_event(id, GameEvent::Command(response));
    }

    /// Moves the server time ahead by one tick, as defined in the config file.
    fn tick(&mut self) {
        self.timer.count += 1;

        if self.timer.count % self.timer.tick_rate == 0 {
            self.timer.seconds += 1;
        }

        // Runs the server at 20 ticks per second
        sleep(Duration::from_millis(self.timer.interval));
    }

    // Adds a system to the world with mutable access to both the world and its parent struct.
    // Systems run once per server tick and run in the order they were added.
    pub fn add_system(&mut self, name: &'static str, system: impl System + 'static) -> &mut Self {
        self.systems
            .write
            .push(SystemHandle::new(name, Box::new(system)));

        self
    }

    // Adds a system to the world with read-only access to both the world and its parent struct.
    // Systems run once per server tick and run in the order they were added.
    pub fn add_system_readonly(
        &mut self,
        name: &'static str,
        system: impl SystemReadOnly + 'static,
    ) -> &mut Self {
        self.systems
            .readonly
            .push(SystemReadOnlyHandle::new(name, Box::new(system)));

        self
    }

    /// Adds a command to the world. Commands are invoked by peers when sending a message and run
    /// once on the next frame.
    pub fn add_command(
        &mut self,
        command: Command,
        func: impl FnMut(Context) -> Result<Response> + Send + Sync + 'static,
    ) -> &mut Self {
        // Combine the command name and aliases into a single array so we can map them later.
        let mut keys: Vec<String> = command
            .aliases
            .iter()
            .map(|s| s.to_lowercase())
            .collect::<_>();
        keys.push(command.name.to_lowercase());

        // Create the handle; we move the command and func at this point.
        self.commands.push(CommandHandle {
            inner: command,
            func: Box::new(func),
        });

        // Create a mapping from each key (name or alias) to the index that the command handle is
        // stored at. This lets us access commands later simply by indexing in. This is much easier
        // than maintaining a map with multiple handles (which would require cloning or a lot of
        // ref handling - which would make the World struct more verbose to invoke for users).
        let map_index = self.commands.len() - 1;
        for key in keys {
            self.command_map.insert(key, map_index);
        }

        self
    }

    /// Helper function for sending a prompt to the client.
    fn send_prompt(&mut self, id: PlayerId) {
        // I'm not happy with this structure, but we always want to send a prompt after
        // a command invocation, so we just send a second message with the prompt. It is
        // more idiomatic than handling this in the command itself.
        if let Ok(player) = self.get_player(id) {
            self.send_event(
                id,
                GameEvent::Command(Response::Client(format!("{}", Prompt::from(player)))),
            )
        }
    }

    pub fn spawn_monster(&mut self, template_key: &str, position: Vec3) -> Option<EntityId> {
        let template = self.monsters.get_template(&template_key);

        if let Some(template) = template {
            let new_monster = template.clone();
            let id = self.next_id();
            let mut monster = Monster::new(id, new_monster);
            monster.with_position(position);

            self.monsters.insert(monster);

            return Some(id);
        }

        None
    }

    /// Gets a player by their ID from world state.
    pub fn get_player(&self, id: PlayerId) -> Result<&Player> {
        match self.players.iter().find(|p| p.id == id) {
            Some(p) => Ok(p),
            None => Err(Error {
                kind: ErrorType::Internal,
                message: "Player not found".to_string(),
            }),
        }
    }

    /// Gets a mutable player by their ID from world state.
    pub fn get_player_mut(&mut self, id: PlayerId) -> Result<&mut Player> {
        match self.players.iter_mut().find(|p| p.id == id) {
            Some(p) => Ok(p),
            None => Err(Error {
                kind: ErrorType::Internal,
                message: "Player not found".to_string(),
            }),
        }
    }

    /// Gets a group of players by their IDs from the world state.
    pub fn get_players(&self, ids: &[PlayerId]) -> Result<Vec<&Player>> {
        let mut players = Vec::with_capacity(ids.len());

        for id in ids {
            players.push(self.get_player(*id)?);
        }

        Ok(players)
    }

    /// Gets all players currently in the world.
    pub fn get_all_players(&self) -> Result<Vec<&Player>> {
        Ok(self.players.iter().collect())
    }

    pub fn get_monster(&self, id: EntityId) -> Result<&Monster> {
        match self.monsters.iter().find(|m| m.id == id) {
            Some(m) => Ok(m),
            None => Err(Error {
                kind: ErrorType::Internal,
                message: "Monster not found".to_string(),
            }),
        }
    }

    pub fn get_monsters(&self, position: Vec3) -> Vec<&Monster> {
        self.monsters
            .iter()
            .filter(|m| m.position == position)
            .collect::<Vec<_>>()
    }

    pub fn next_id(&mut self) -> EntityId {
        self.active_entities += 1;
        self.spawned_entities += 1;
        EntityId {
            id: self.spawned_entities,
            tick: self.timer.count,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!("Blossom World Stats\nUptime: {}\nAverage Execution Time: {}\nConnections: {}\nSystems: {}\nEntity Count: {} active, {} spawned\n{}",
            self.timer.to_string().bold(),
            self.systems.execution_timer.average().foreground(GREEN).bold(),
            self.players.len().to_string().foreground(GREEN).bold(),
            self.systems,
            self.active_entities.to_string().foreground(GREEN).bold(),
            self.spawned_entities.to_string().foreground(YELLOW).bold(),
            self.monsters
        );

        write!(f, "{output}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_single_player() {
        let mut world = World::new();
        let player = Player::new(1);
        world.players.insert(player.clone());

        assert_eq!(world.get_player(1).unwrap(), &player);
    }

    #[test]
    fn get_multiple_players() {
        let mut world = World::new();
        let player1 = Player::new(1);
        let player2 = Player::new(2);
        let player3 = Player::new(3);
        world.players.insert(player1.clone());
        world.players.insert(player2.clone());
        world.players.insert(player3.clone());

        let players = world.get_players(&[1, 3]).unwrap();

        assert_eq!(players.len(), 2);
        assert_eq!(players[0], &player1);
        assert_eq!(players[1], &player3);
    }

    #[test]
    fn get_all_players() {
        let mut world = World::new();
        let player1 = Player::new(1);
        let player2 = Player::new(2);
        let player3 = Player::new(3);
        world.players.insert(player1.clone());
        world.players.insert(player2.clone());
        world.players.insert(player3.clone());

        let players = world.get_all_players().unwrap();

        assert_eq!(players.len(), 3);
        assert_eq!(players[0], &player1);
        assert_eq!(players[1], &player2);
        assert_eq!(players[2], &player3);
    }

    #[test]
    fn add_command() {
        let mut world = World::new();
        let command = Command::new("test".to_string());
        let func = |_: Context| -> Result<Response> { Ok(Response::Empty) };

        world.add_command(command, func);

        assert_eq!(world.commands.len(), 1);
        assert_eq!(world.commands[0].inner.name, "test");
    }

    #[test]
    fn add_system() {
        let mut world = World::new();

        struct TestSystem {
            count: u8,
        }

        impl System for TestSystem {
            fn update(&mut self, _: &mut World) {
                self.count += 1;
            }
        }

        world.add_system("test_system", TestSystem { count: 0 });

        assert_eq!(world.systems.write.len(), 1);
        assert_eq!(world.systems.write[0].name, "test_system");
    }

    #[test]
    fn add_system_readonly() {
        let mut world = World::new();

        struct TestSystem;

        impl SystemReadOnly for TestSystem {
            fn update(&self, _: &World) {}
        }

        world.add_system_readonly("test_system", TestSystem);

        assert_eq!(world.systems.readonly.len(), 1);
        assert_eq!(world.systems.readonly[0].name, "test_system");
    }

    #[test]
    fn advance_tick() {
        let mut world = World::new();
        world.tick();

        assert_eq!(world.timer.count, 1);
    }
}
