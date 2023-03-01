use iridescent::{Styled, StyledString};
use serde::Deserialize;

use crate::{
    direction::Direction,
    entity::{Entity, EntityId},
    player::PlayerId,
    quickmap::QuickMapKey,
    theme,
    vec3::Vec3,
    world::World,
};

#[derive(Debug)]
pub struct Room {
    pub entity_id: EntityId,
    pub name: String,
    pub position: Vec3,
    pub description: String,
    pub exits: Vec<Direction>,
    pub mob_pool: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RoomBuilder {
    pub area: String,
    pub name: String,
    pub description: String,
    pub position: Vec3,
    pub exits: Vec<Direction>,
    pub mob_pool: Vec<String>,
}

impl RoomBuilder {
    pub fn build(self, id: EntityId) -> Room {
        Room {
            entity_id: id,
            name: self.name,
            position: self.position,
            description: self.description,
            exits: self.exits,
            mob_pool: self.mob_pool,
        }
    }
}

impl Room {
    /// Returns the name of the room as a styled string.
    pub fn name(&self) -> StyledString {
        self.name.foreground(theme::GREEN)
    }

    /// Returns all of the exits in the room as a styled string.
    pub fn exits(&self) -> StyledString {
        let exit_string =
            self.exits.iter().map(|exit| format!("{exit}")).collect::<Vec<String>>().join(", ");

        let exits = format!("[Exits: {exit_string}]");

        exits.foreground(theme::GREEN)
    }

    /// Returns a view of the room; this includes the room name, description,
    /// exits, nearby players, and nearby items, objects, enemies, and anything
    /// else relevant to the room. Generally invoked when using commands like
    /// `look` or when moving into a room.
    pub fn view(&self, id: PlayerId, world: &World) -> String {
        let player = world.get_player(id);

        if let Ok(player) = player {
            let mut text = String::new();

            // We always display the room name first.
            text.push_str(&format!("{}", self.name()));

            // Add the room description if force is true. This is true when the
            // `look` command is called explicitly by a player, as opposed to
            // implicitly when the walk command is used.
            if !player.brief {
                text.push_str(&format!("\n{}\n", self.description));
            }

            // Display any monsters in this room.
            let monsters_here = world
                .monsters
                .iter()
                .filter(|m| m.position == player.position)
                .map(|m| format!("{}", m.name.clone().foreground(theme::RED).bold()))
                .collect::<Vec<_>>()
                .join(", ");

            if !monsters_here.is_empty() {
                text.push_str(&format!("\nNearby you see a {monsters_here}.\n"));
            }

            // Get all players in the players current room except the current
            // player.
            let players_here_list = world
                .players
                .iter()
                .filter(|p| p.position == player.position && p.id != id)
                .map(|p| p.name.clone())
                .collect::<Vec<_>>();

            // If there is anyone else in the room, we add them to the text.
            if !players_here_list.is_empty() {
                text.push_str(&format!(
                    "\nPlayers here: {}\n",
                    if players_here_list.len() > 4 {
                        format!(
                            "{}, and {} others",
                            players_here_list[0..4].join(", "),
                            players_here_list.len().saturating_sub(4)
                        )
                    } else {
                        players_here_list.join(", ")
                    }
                ));
            }

            // Add the exits list.
            text.push_str(&format!("\n{}", self.exits()));

            return text;
        }

        "This room has no description.".to_string()
    }
}

impl QuickMapKey<Vec3> for Room {
    fn key(&self) -> Vec3 {
        self.position
    }
}

impl Entity for Room {
    fn id(&self) -> EntityId {
        self.entity_id
    }
}
