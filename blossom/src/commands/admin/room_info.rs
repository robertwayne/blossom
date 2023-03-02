use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::{ErrorType, Result},
    prelude::{Error, World},
    response::Response,
    role::Role,
};

pub struct RoomInfo;

impl GameCommand for RoomInfo {
    fn create() -> Command {
        Command {
            name: "@room",
            description: "Information about the current room.",
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let binding = ctx.world.players.read();
        let Some(player) = binding.get(&ctx.id) else {
            return Err(Error::new(ErrorType::Internal, "Player not found."));
        };

        if player.account.roles.contains(&Role::Admin) {
            let binding = ctx.world.rooms.read();
            let Some(room) = binding.get(&player.position) else {
                return Err(Error::new(ErrorType::Internal, "Room not found."));
            };

            Ok(Response::client_message(format!("{room:#?}",)))
        } else {
            World::unknown(player.id)
        }
    }
}
