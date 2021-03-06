use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
};

pub struct Afk;

impl GameCommand for Afk {
    fn create() -> Command {
        Command {
            name: "afk".to_string(),
            description: "Marks you as AFK.".to_string(),
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let player = ctx.world.get_player_mut(ctx.id)?;

        player.afk = !player.afk;
        player.dirty = true;

        Ok(Response::Client(format!(
            "AFK mode is now {}.",
            if player.afk { "on" } else { "off" }
        )))
    }
}
