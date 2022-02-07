use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
};

pub struct Look;

impl GameCommand for Look {
    fn create() -> Command {
        Command {
            name: "look".to_string(),
            description: "Lists all online players in the game.".to_string(),
            aliases: Vec::from(["l".to_string()]),
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let player = ctx.world.get_player(ctx.id)?;

        let view = ctx
            .world
            .rooms
            .iter()
            .find(|r| r.position == player.position)
            .map(|r| r.view(player.id, ctx.world));

        if let Some(view) = view {
            Ok(Response::Client(view))
        } else {
            Ok(Response::Client(
                "You are lost in the void. There is nowhere to go.".to_string(),
            ))
        }
    }
}
