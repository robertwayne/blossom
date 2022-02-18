use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
};

pub struct Say;

impl GameCommand for Say {
    fn create() -> Command {
        Command {
            name: "say".to_string(),
            description: "Sends a message to all players in the game.".to_string(),
            aliases: Vec::from([",".to_string()]),
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let message = ctx.input.args.join(" ");

        if message.is_empty() {
            return Ok(Response::Empty);
        }

        let verb = if message.ends_with('?') {
            "asks"
        } else if message.ends_with('!') {
            "exclaims"
        } else if message.ends_with("!!!") {
            "screams"
        } else {
            "says"
        };

        let player = ctx.world.get_player_mut(ctx.id)?;
        let name = player.name.as_str();
        let position = player.position;

        let msg = format!("{} {}, \"{}\"", name, verb, message,);

        let players_in_room = ctx
            .world
            .players
            .iter()
            .filter(|p| p.position == position)
            .map(|p| p.id)
            .collect::<Vec<_>>();

        Ok(Response::Channel(players_in_room, msg))
    }
}
