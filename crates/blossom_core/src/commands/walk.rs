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
            name: "north".to_string(),
            description: "Moves you in the specified direction.".to_string(),
            aliases: vec![
                "south".to_string(),
                "east".to_string(),
                "west".to_string(),
                "up".to_string(),
                "down".to_string(),
                "n".to_string(),
                "s".to_string(),
                "e".to_string(),
                "w".to_string(),
                "u".to_string(),
                "d".to_string(),
            ],
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let direction = Direction::from(ctx.input.command);
        let player = ctx.world.get_player(ctx.id)?;
        let current_room = ctx
            .world
            .rooms
            .iter()
            .find(|r| r.position == player.position);

        if let Some(current_room) = current_room {
            // If the current room cannot be exited in the given direction, we just break early and
            // let the player know.
            if !current_room.exits.contains(&direction) {
                return Ok(Response::Client(format!(
                    "You can't go {} from here.",
                    direction
                )));
            }

            // We get all the players in the current room and broadcast a 'leaving' message to them.
            let players_here = ctx
                .world
                .players
                .iter()
                .filter(|p| p.position == player.position && p.id != player.id)
                .map(|p| p.id)
                .collect();

            // Append '-wards' to the direction (eg. 'downwards' or 'upwards') for a more natural
            // sounding message.
            let formatted_direction: String =
                if direction == Direction::Up || direction == Direction::Down {
                    format!("{}wards", direction)
                } else {
                    format!("{}", direction)
                };

            ctx.world.send_event(
                player.id,
                GameEvent::Command(Response::Channel(
                    players_here,
                    format!("{} leaves {}.", player.name, formatted_direction),
                )),
            );

            let new_position = Vec3::from(direction) + player.position;

            // We get all the players in the new room and broadcast a 'entering' message to them.
            let players_there = ctx
                .world
                .players
                .iter()
                .filter(|p| p.position == new_position && p.id != player.id)
                .map(|p| p.id)
                .collect();

            // Modify the message based on the direction the player is moving; similarly to above, we
            // adjust 'up' and 'down' to use 'climbs' instead of 'walks' for a natural sounding message.
            let broadcast_message: String = match direction {
                Direction::Up => format!("{} climbs up from below.", player.name),
                Direction::Down => format!("{} climbs down from above.", player.name),
                _ => format!("{} walks {}.", player.name, formatted_direction),
            };

            ctx.world
                .send_command(ctx.id, Response::Channel(players_there, broadcast_message));

            // We have to get a new mutable reference to the player here because we're mutating this
            // time, and we can't start with a mutable reference as it is immutably borrowed many
            // times throughout the rest of the function.
            //
            // We know the player exists as we did get a reference to it in the first place.
            let player = ctx
                .world
                .players
                .iter_mut()
                .find(|p| p.id == ctx.id)
                .expect("This should never happen.");
            player.position = new_position;
            player.dirty = true;

            let room_view = ctx
                .world
                .rooms
                .iter()
                .find(|r| r.position == player.position)
                .map(|r| r.view(ctx.id, ctx.world));

            if let Some(view) = room_view {
                return Ok(Response::Client(view));
            }
        }

        Ok(Response::Client(
            "You are lost in the void. There is nowhere to go.".to_string(),
        ))
    }
}
