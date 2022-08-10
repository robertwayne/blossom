use crate::{context::Context, error::Result, response::Response, role::Role};

pub trait GameCommand {
    fn create() -> Command;
    fn run(ctx: Context) -> Result<Response>;
}

/// Wrapper around a Command which provides access to its `run` method via the `func` field.
/// In generaly, you should never have to work with `CommandHandle`s directly, always work with
/// the `Command` struct and `GameCommand` trait implementations.
pub struct CommandHandle {
    pub func: Box<dyn FnMut(Context) -> Result<Response> + Send + Sync + 'static>,
    pub inner: Command,
}

/// Any text command that can be invoked by the player with its key words. This is how the players
/// interact with the game generally. Commands can be thought of similar to `systems` that are run
/// once on the next frame. A command should ALWAYS return a meaningful Response to the player.
#[derive(Debug, Default)]
pub struct Command {
    pub name: String,
    pub arguments: Vec<String>,
    pub description: String,
    pub aliases: Vec<String>,
    pub permissions: Vec<Role>,
}

impl Command {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}
