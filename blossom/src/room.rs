use iridescent::{Styled, StyledString};
use serde::Deserialize;

use crate::{
    direction::Direction,
    entity::{Entity, EntityId},
    player::PlayerId,
    quickmap::QuickMapKey,
    theme,
    utils::as_comma_separated_list,
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
        let exit_string = as_comma_separated_list(&self.exits);
        let exits = format!("[Exits: {exit_string}]");

        exits.foreground(theme::GREEN)
    }

    /// Returns a view of the room; this includes the room name, description,
    /// exits, nearby players, and nearby items, objects, enemies, and anything
    /// else relevant to the room. Generally invoked when using commands like
    /// `look` or when moving into a room.
    pub fn view(&self, id: PlayerId, world: &World) -> String {
        let Ok(player) = world.get_player(id) else {
            return "This room has no description.".to_string();
        };

        // We always display the room name first.
        let mut text = String::new();
        text.push_str(&format!("{}", self.name()));

        // Add the room description if force is true. This is true when the
        // `look` command is called explicitly by a player, as opposed to
        // implicitly when the walk command is used.
        if !player.brief {
            text.push_str(&format!("\n{}\n", self.description));
        }

        // Display any monsters in this room.
        let monster_list = world
            .monsters
            .iter()
            .filter_map(|m| {
                if m.position == player.position {
                    Some(format!("{}", m.name.clone().foreground(theme::RED).bold()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !monster_list.is_empty() {
            let monsters = as_comma_separated_list(&monster_list);
            text.push_str(&format!("\nNearby you see a {monsters}.\n"))
        }

        // Get all players in the players current room except the current
        // player.
        let player_list = world
            .players
            .iter()
            .filter_map(|p| {
                if p.position == player.position && p.id != id {
                    Some(p.name.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // If there is anyone else in the room, we add them to the text.
        if !player_list.is_empty() {
            let names = if player_list.len() > 4 {
                // If there are more than 4 players in the room, we only display
                // the first 4 and then the count of how many others.
                let first = player_list.get(..4).unwrap().join(", ");
                let remaining = player_list.len().saturating_sub(4);

                if remaining == 1 {
                    format!("{first}, and 1 other.")
                } else {
                    format!("{first}, and {remaining} others.")
                }
            } else {
                // Otherwise, we just display all the names.
                as_comma_separated_list(&player_list)
            };

            text.push_str(&format!("\nPlayers here: {names}\n"));
        }

        // Add the exits.
        text.push_str(&format!("\n{}", self.exits()));

        text
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
