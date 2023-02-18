use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
    role::Role,
    world::World,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Version;

impl GameCommand for Version {
    fn create() -> Command {
        Command {
            name: "@version",
            description: "Shows the internal version the game is running on.",
            permissions: vec![Role::Admin],
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let player = ctx.world.get_player_mut(ctx.id)?;
        if !player.account.roles.contains(&Role::Admin) {
            return World::unknown(player.id);
        }

        Ok(Response::Client(VERSION.to_string()))
    }
}
