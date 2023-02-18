use iridescent::Styled;

use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
    theme,
};

pub struct GlobalChat;

impl GameCommand for GlobalChat {
    fn create() -> Command {
        Command {
            name: "global",
            description: "Sends a message to all players in the game.",
            aliases: vec!["ooc"],
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

        let msg = format!(
            "{} {} {}, \"{}\"",
            "[Global]".foreground(theme::GREEN),
            player.name,
            verb,
            message,
        );

        let all = ctx.world.players.iter().map(|p| p.id).collect::<Vec<_>>();

        Ok(Response::Channel(all, msg))
    }
}
