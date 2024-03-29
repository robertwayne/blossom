use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::{ErrorType, Result},
    prelude::Error,
    response::Response,
};

pub struct Look;

impl GameCommand for Look {
    fn create() -> Command {
        Command {
            name: "look",
            description: "Lists all online players in the game.",
            aliases: Vec::from(["l"]),
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let binding = ctx.world.players.read();
        let Some(player) = binding.get(&ctx.id) else {
            return Err(Error::new(ErrorType::Internal, "Player not found."));
        };
        let args = ctx.args();

        // Check if the player is looking at a monster.
        if args.get(0).is_some() {
            let monsters = ctx.world.get_monsters(player.position);
            let index = ctx.input.fuzzy_match(&monsters[..]);

            if let Some(index) = index {
                if let Some(monster) = monsters.get(index) {
                    return Ok(Response::client_message(format!("{monster}")));
                }
            }

            return Ok(Response::client_message("Monster not found."));
        }

        let view = ctx.world.rooms.read().iter().find_map(|r| {
            if r.position == player.position {
                Some(r.view(player.id, ctx.world))
            } else {
                None
            }
        });

        if let Some(view) = view {
            Ok(Response::client_message(view))
        } else {
            Ok(Response::client_message(
                "You are lost in the void. There is nowhere to go.",
            ))
        }
    }
}
