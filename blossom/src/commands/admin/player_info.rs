use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
    role::Role,
    world::World,
};

pub struct PlayerInfo;

impl GameCommand for PlayerInfo {
    fn create() -> Command {
        Command {
            name: "@player",
            description: "Information about a player.",
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let player = ctx.world.get_player(ctx.id)?;

        if player.account.roles.contains(&Role::Admin) {
            match ctx.input.args.get(0) {
                Some(name) => {
                    if let Some(target) = ctx.world.players.iter().find(|p| p.name == *name) {
                        return Ok(Response::client_message(format!("{target:#?}",)));
                    }

                    Ok(Response::client_message("Player not found."))
                }
                None => {
                    Ok(Response::client_message("View player information. Usage: @player <name>"))
                }
            }
        } else {
            World::unknown(player.id)
        }
    }
}
