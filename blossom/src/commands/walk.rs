use crate::{
    command::{Command, GameCommand},
    context::Context,
    direction::Direction,
    error::{ErrorType, Result},
    event::GameEvent,
    prelude::Error,
    response::Response,
    vec3::Vec3,
};

const LOST_MESSAGE: &str = "You are lost in the void. There is nowhere to go.";

pub struct Walk;

impl GameCommand for Walk {
    fn create() -> Command {
        Command {
            name: "north",
            description: "Moves you in the specified direction.",
            aliases: vec![
                "south", "east", "west", "up", "down", "n", "s", "e", "w", "u", "d",
            ],
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let direction = Direction::from(ctx.input.command);

        // We get the player's name, id and position so we can use them later.
        // We do this inside a tight scope so we can borrow the world
        // immediately after.
        let (player_name, player_id, player_position) = {
            let binding = ctx.world.players.write();
            let Some(player) = binding.get(&ctx.id) else {
            return Err(Error::new(ErrorType::Internal, "Player not found."));
        };
            (player.name.clone(), player.id, player.position)
        };

        let r = ctx.world.rooms.read();
        let Some(current_room) = r
            .iter()
            .find(|r| r.position == player_position) else {
            return Ok(Response::client_message(LOST_MESSAGE));
        };

        // If the current room cannot be exited in the given direction, we
        // just break early and let the player know.
        if !current_room.exits.contains(&direction) {
            return Ok(Response::client_message(format!(
                "You can't go {direction} from here."
            )));
        }

        // We get all the players in the current room and broadcast a
        // 'leaving' message to them.
        let players_here = ctx
            .world
            .players
            .read()
            .iter()
            .filter_map(|p| {
                if p.position == player_position && p.id != player_id {
                    Some(p.id)
                } else {
                    None
                }
            })
            .collect();

        // Append '-wards' to the direction (eg. 'downwards' or 'upwards')
        // for a more natural sounding message.
        let formatted_direction: String =
            if direction == Direction::Up || direction == Direction::Down {
                format!("{direction}wards")
            } else {
                format!("{direction}")
            };

        ctx.world.send_event(
            player_id,
            GameEvent::Command(Response::Channel(
                players_here,
                format!("{player_name} walks {formatted_direction}."),
            )),
        );

        let new_position = Vec3::from(direction) + player_position;

        // We get all the players in the new room and broadcast a 'entering'
        // message to them.
        let players_in_next_room = ctx
            .world
            .players
            .read()
            .iter()
            .filter_map(|p| {
                if p.position == new_position && p.id != player_id {
                    Some(p.id)
                } else {
                    None
                }
            })
            .collect();

        // Modify the message based on the direction the player is moving;
        // similarly to above, we adjust 'up' and 'down' to use 'climbs'
        // instead of 'walks' for a natural sounding message.
        let broadcast_message = match direction {
            Direction::Up => format!("{player_name} climbs up from below."),
            Direction::Down => format!("{player_name} climbs down from above."),
            _ => format!("{player_name} walks in from the {formatted_direction}."),
        };

        ctx.world.send_command(
            ctx.id,
            Response::Channel(players_in_next_room, broadcast_message),
        );

        // We have to get a new mutable reference to the player here, as we
        // only needed an immutable reference in the beginning, which
        // couldn't be held while we were borrowing the world mutably.
        //
        // We need to rework the way the world works, probably with some
        // serious interior mutability, to avoid this.
        if let Some(player) = ctx.world.players.write().get_mut(&ctx.id) {
            player.position = new_position;
            player.dirty = true;
        }

        let Some(view) = ctx.world.rooms.read().iter().find_map(|r| {
            if r.position == new_position {
                Some(r.view(player_id, ctx.world))
            } else {
                None
            }
        }) else {
            return Ok(Response::client_message(LOST_MESSAGE));
        };

        Ok(Response::client_message(view))
    }
}
