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
            name: "@world".to_string(),
            description: "Information about the world state.".to_string(),
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let player = ctx.world.get_player(ctx.id)?;

        if player.account.roles.contains(&Role::Admin) {
            Ok(Response::Client(format!("{}", ctx.world)))
        } else {
            World::unknown(player.id)
        }
    }
}
