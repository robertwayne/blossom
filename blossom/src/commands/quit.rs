use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
};

pub struct Quit;

impl GameCommand for Quit {
    fn create() -> Command {
        Command {
            name: "quit",
            description: "Quits the game.",
            aliases: vec!["logout", "exit"],
            ..Default::default()
        }
    }

    fn run(_ctx: Context) -> Result<Response> {
        Ok(Response::Close)
    }
}
