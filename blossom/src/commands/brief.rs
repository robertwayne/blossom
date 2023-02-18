use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
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
        let player = ctx.world.get_player_mut(ctx.id)?;

        player.brief = !player.brief;
        player.dirty = true;

        Ok(Response::client_message(format!(
            "Brief mode is now {}.",
            if player.brief { "on" } else { "off" }
        )))
    }
}
