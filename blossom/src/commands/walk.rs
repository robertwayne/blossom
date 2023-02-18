use crate::{
    command::{Command, GameCommand},
    context::Context,
    direction::Direction,
    error::Result,
    event::GameEvent,
    response::Response,
    vec3::Vec3,
};

pub struct Walk;

impl GameCommand for Walk {
    fn create() -> Command {
        Command {
            name: "north",
            description: "Moves you in the specified direction.",
            aliases: vec!["south", "east", "west", "up", "down", "n", "s", "e", "w", "u", "d"],
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let direction = Direction::from(ctx.input.command);

        // We get the player's name, id and position so we can use them later.
        // We do this inside a tight scope so we can borrow the world
        // immediately after.
        let (player_name, player_id, player_position) = {
            let player = ctx.world.get_player(ctx.id)?;
            (player.name.clone(), player.id, player.position)
        };

        if let Some(room) = ctx.world.rooms.iter().find(|r| r.position == player_position) {
            // If the current room cannot be exited in the given direction, we
            // just break early and let the player know.
            if !room.exits.contains(&direction) {
                return Ok(Response::client_message(format!(
                    "You can't go {direction} from here."
                )));
            }

            // We get all the players in the current room and broadcast a
            // 'leaving' message to them.
            let players_here = ctx
                .world
                .players
                .iter()
                .filter(|p| p.position == player_position && p.id != player_id)
                .map(|p| p.id)
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
                    format!("{} leaves {formatted_direction}.", player_name),
                )),
            );

            let new_position = Vec3::from(direction) + player_position;

            // We get all the players in the new room and broadcast a 'entering'
            // message to them.
            let players_in_next_room = ctx
                .world
                .players
                .iter()
                .filter(|p| p.position == new_position && p.id != player_id)
                .map(|p| p.id)
                .collect();

            // Modify the message based on the direction the player is moving;
            // similarly to above, we adjust 'up' and 'down' to use 'climbs'
            // instead of 'walks' for a natural sounding message.
            let broadcast_message = match direction {
                Direction::Up => format!("{} climbs up from below.", player_name),
                Direction::Down => format!("{} climbs down from above.", player_name),
                _ => format!("{} walks {}.", player_name, formatted_direction),
            };

            ctx.world
                .send_command(ctx.id, Response::Channel(players_in_next_room, broadcast_message));

            // We have to get a new mutable reference to the player here, as we
            // only needed an immutable reference in the beginning, which
            // couldn't be held while we were borrowing the world mutably.
            //
            // We need to rework the way the world works, probably with some
            // serious interior mutability, to avoid this.
            if let Some(player) = ctx.world.players.get_mut(&ctx.id) {
                player.position = new_position;
                player.dirty = true;
            }

            if let Some(view) = ctx
                .world
                .rooms
                .iter()
                .find(|r| r.position == player_position)
                .map(|r| r.view(ctx.id, ctx.world))
            {
                return Ok(Response::client_message(view));
            }
        }

        Ok(Response::client_message("You are lost in the void. There is nowhere to go."))
    }
}
