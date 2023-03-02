use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::{ErrorType, Result},
    prelude::Error,
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
        let binding = ctx.world.players.read();
        let Some(player) = binding.get(&ctx.id) else {
            return Err(Error::new(ErrorType::Internal, "Player not found."));
        };

        if player.account.roles.contains(&Role::Admin) {
            Ok(Response::client_message(format!("{}", ctx.world)))
        } else {
            World::unknown(player.id)
        }
    }
}
