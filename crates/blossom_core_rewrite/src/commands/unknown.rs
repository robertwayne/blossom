use crate::{error::Result, player::PlayerId, response::Response};

pub fn unknown_command(_id: PlayerId) -> Result<Response> {
    Ok(Response::Client(
        "Unknown command. Type \"help\" for a list of commands.".to_string(),
    ))
}
