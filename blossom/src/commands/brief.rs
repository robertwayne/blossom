use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::{ErrorType, Result},
    prelude::Error,
    response::Response,
};

pub struct Brief;

impl GameCommand for Brief {
    fn create() -> Command {
        Command {
            name: "brief",
            description: "Lists all online players in the game.",
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let mut binding = ctx.world.players.write();
        let Some(player) = binding.get_mut(&ctx.id) else {
            return Err(Error::new(ErrorType::Internal, "Player not found."));
        };

        player.brief = !player.brief;
        player.dirty = true;

        Ok(Response::client_message(format!(
            "Brief mode is now {}.",
            if player.brief { "on" } else { "off" }
        )))
    }
}
