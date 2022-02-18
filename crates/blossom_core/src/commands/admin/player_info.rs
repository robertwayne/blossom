use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
    role::Role,
};

pub struct PlayerInfo;

impl GameCommand for PlayerInfo {
    fn create() -> Command {
        Command {
            name: "@player".to_string(),
            description: "Information about a player.".to_string(),
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let player = ctx.world.get_player(ctx.id)?;

        if player.account.roles.contains(&Role::Admin) {
            match ctx.input.args.get(0) {
                Some(name) => {
                    if let Some(target) = ctx.world.players.iter().find(|p| p.name == *name) {
                        return Ok(Response::Client(format!("{:#?}", target)));
                    }

                    Ok(Response::Client("Player not found.".to_string()))
                }
                None => Ok(Response::Client(
                    "View player information. Usage: @player <name>".to_string(),
                )),
            }
        } else {
            ctx.world.unknown(player.id)
        }
    }
}
