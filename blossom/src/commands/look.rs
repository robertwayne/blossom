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
        let args = ctx.args();

        // Check if the player is looking at a monster.
        if args.get(0).is_some() {
            let monsters = ctx.world.get_monsters(player.position);
            let index = ctx.input.fuzzy_match(&monsters[..]);

            if let Some(index) = index {
                let monster = &monsters[index];

                return Ok(Response::Client(format!("{monster}")));
            }

            return Ok(Response::Client("Monster not found.".to_string()));
        }

        let view = ctx
            .world
            .rooms
            .iter()
            .find(|r| r.position == player.position)
            .map(|r| r.view(player.id, ctx.world));

        if let Some(view) = view {
            Ok(Response::Client(view))
        } else {
            Ok(Response::Client("You are lost in the void. There is nowhere to go.".to_string()))
        }
    }
}
