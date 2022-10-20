use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
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
            name: "@help".to_string(),
            description: "Shows the admin help menu.".to_string(),
            permissions: vec![Role::Admin],
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let player = ctx.world.get_player_mut(ctx.id)?;
        if !player.account.roles.contains(&Role::Admin) {
            return World::unknown(player.id);
        }

        Ok(Response::Client(HELP_TEXT.to_string()))
    }
}
