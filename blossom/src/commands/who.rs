use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
};

pub struct Who;

impl GameCommand for Who {
    fn create() -> Command {
        Command {
            name: "who",
            description: "Lists all online players in the game.",
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let players = ctx
            .world
            .players
            .iter()
            .map(|p| if p.afk { format!("{} (AFK)", p.name) } else { p.name.clone() })
            .collect::<Vec<_>>();

        Ok(Response::Client(format!("Online: {}", players.join(", "))))
    }
}
