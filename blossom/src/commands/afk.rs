use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::{ErrorType, Result},
    prelude::Error,
    response::Response,
};

pub struct Afk;

impl GameCommand for Afk {
    fn create() -> Command {
        Command {
            name: "afk",
            description: "Marks you as AFK.",
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let mut binding = ctx.world.players.write();
        let Some(player) = binding.get_mut(&ctx.id) else {
            return Err(Error::new(ErrorType::Internal, "Player not found."));
        };

        player.afk = !player.afk;
        player.dirty = true;

        Ok(Response::client_message(format!(
            "AFK mode is now {}.",
            if player.afk { "on" } else { "off" }
        )))
    }
}
