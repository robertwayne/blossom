use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::{ErrorType, Result},
    prelude::Error,
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
        let binding = ctx.world.players.read();
        let Some(player) = binding.get(&ctx.id) else {
            return Err(Error::new(ErrorType::Internal, "Player not found."));
        };
        if !player.account.roles.contains(&Role::Admin) {
            return World::unknown(player.id);
        }

        Ok(Response::client_message(VERSION))
    }
}
