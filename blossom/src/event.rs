use flume::Sender;

use crate::{
    input::Input,
    player::{Player, PlayerId},
    response::Response,
};

#[derive(Debug)]
pub enum Event {
    // Events propgated from the world to the broker. The connection loop parses
    // these events.
    Game(PlayerId, GameEvent),
    // Events propgated from the connection loop to the broker. The game loop
    // parses these events.
    Client(PlayerId, ClientEvent),
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Game(id, game) => write!(f, "Game: [{id}] {game:#?}"),
            Event::Client(id, client) => write!(f, "Client: [{id}] {client:#?}"),
        }
    }
}

/// Represents an event SENT BY THE GAME. A `GameEvent` will never be read by
/// the game itself, and is generally handled by the connection loop. In some
/// cases, the broker may respond to one of these events (such as in the case of
/// Save or `GlobalSave`, because the broker holds the peer map).
#[derive(Clone, Debug)]
pub enum GameEvent {
    // Returns a response from a successful ClientEventType::Connect
    Accepted(Response),
    // Returns a response from a successful ClientEventType::Command
    Command(Response),
    // Returns a response from a successful ClientEventType::Ping
    Pong(Response),
    // A manually called event that saves a single player to the database
    Save(Player),
    // An interval-based event that saves all active players to the database
    GlobalSave(Vec<Player>),
}

impl std::fmt::Display for ClientEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientEvent::Connect(p, _) => write!(f, "Connect {}", p.id),
            ClientEvent::Command(t) => write!(f, "Command {t}"),
            ClientEvent::Ping => write!(f, "Ping"),
            ClientEvent::Disconnect => write!(f, "Disconnect"),
        }
    }
}

/// Represents an event SENT BY THE CLIENT. A `ClientEvent` will never be read
/// by the client itself, and is generally handled by the game loop. Note that
/// when the word Client is used, this refers to events created and sent from
/// the conncetion loop after parsing a valid packet received from a peer.
#[derive(Debug)]
pub enum ClientEvent {
    // Post-authentication event that adds a player to the world
    Connect(Player, Option<Sender<Event>>),
    // Manually called event that removes a player from the world
    Disconnect,
    // Client-sent command
    Command(Input),
    // An event that pings the server for a response on empty input
    Ping,
}

impl std::fmt::Display for GameEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameEvent::Accepted(response) => write!(f, "Accepted {response}"),
            GameEvent::Command(response) => write!(f, "Command {response}"),
            GameEvent::Pong(response) => write!(f, "Pong {response}"),
            GameEvent::Save(player) => write!(f, "Save {}", player.id),
            GameEvent::GlobalSave(players) => {
                write!(
                    f,
                    "GlobalSave [{}]",
                    players.iter().map(|p| p.id.to_string()).collect::<Vec<_>>().join(", ")
                )
            }
        }
    }
}
