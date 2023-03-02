use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::{ErrorType, Result},
    prelude::Error,
    response::Response,
    role::Role,
    world::World,
};

const HELP_TEXT: &str = r#"
================================================================================
BLOSSOM ADMIN HELP
================================================================================

Commands:
    @help, @?                 - show this help menu
    @version                  - show the server version
    @world                    - display world information
    @player <name>            - display information about a player
    @system <command> <name>  - run a system command
    @shutdown                 - shutdown the server

================================================================================
"#;

pub struct AdminHelp;

impl GameCommand for AdminHelp {
    fn create() -> Command {
        Command {
            name: "@help",
            description: "Shows the admin help menu.",
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

        Ok(Response::client_message(HELP_TEXT))
    }
}
