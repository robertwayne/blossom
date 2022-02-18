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

        if let Some(value) = args.get(0) {
            let monsters = ctx.world.get_monsters(player.position);

            let monster_strings = monsters
                .iter()
                .map(|s| s.name.split_whitespace())
                .collect::<Vec<_>>()
                .into_iter()
                .flatten()
                .map(|s| s.to_string().to_lowercase())
                .collect::<Vec<_>>();

            if monster_strings.contains(&value.to_lowercase()) {
                let monster = monsters
                    .iter()
                    .find(|s| s.name.to_lowercase() == value.to_lowercase());

                match monster {
                    Some(m) => {
                        return Ok(Response::Client(format!("You see a {}.", m.name)));
                    }
                    None => {
                        return Ok(Response::Client(format!(
                            "You do not see a {} here.",
                            value
                        )))
                    }
                }
            }
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
            Ok(Response::Client(
                "You are lost in the void. There is nowhere to go.".to_string(),
            ))
        }
    }
}
