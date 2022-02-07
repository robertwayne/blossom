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
            name: "quit".to_string(),
            description: "Quits the game.".to_string(),
            ..Default::default()
        }
    }

    fn run(_ctx: Context) -> Result<Response> {
        Ok(Response::Close)
    }
}
