use crate::{error::Result, player::PlayerId, response::Response, world::World};

impl World {
    pub fn unknown(_id: PlayerId) -> Result<Response> {
        Ok(Response::Client(
            "Unknown command. Type \"help\" for a list of commands.".to_string(),
        ))
    }
}
