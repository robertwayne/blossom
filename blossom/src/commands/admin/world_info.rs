use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
    role::Role,
    world::World,
};

pub struct WorldInfo;

impl GameCommand for WorldInfo {
    fn create() -> Command {
        Command {
            name: "@world",
            description: "Information about the world state.",
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let player = ctx.world.get_player(ctx.id)?;

        if player.account.roles.contains(&Role::Admin) {
            Ok(Response::client_message(format!("{}", ctx.world)))
        } else {
            World::unknown(player.id)
        }
    }
}
