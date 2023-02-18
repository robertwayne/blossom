use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
};

const HELP_TEXT: &str = r#"
================================================================================
BLOSSOM HELP
================================================================================

Commands:
    help, ?                 - show this help menu
    look, l                 - display the current rooms description
    n, e, s, w, u, d        - move north, east, south, west, up, down respectively
    say, ,                  - say something in local (room) chat
    ooc, global             - say something in global (world) chat
    who                     - list all online players
    afk                     - toggle AFK mode
    brief                   - toggle brief mode
    exit, logout, quit      - quit the game

================================================================================
"#;

pub struct Help;

impl GameCommand for Help {
    fn create() -> Command {
        Command {
            name: "help",
            description: "Shows this help menu.",
            aliases: vec!["?"],
            ..Default::default()
        }
    }

    fn run(_ctx: Context) -> Result<Response> {
        Ok(Response::Client(HELP_TEXT.to_string()))
    }
}
