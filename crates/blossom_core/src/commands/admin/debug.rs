use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
    role::Role,
    system::SystemStatus,
};

pub struct DebugCommand;

impl GameCommand for DebugCommand {
    fn create() -> Command {
        Command {
            name: "@debug".to_string(),
            description: "Various debugging utilities.".to_string(),
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let player = ctx.world.get_player(ctx.id)?;

        if player.account.roles.contains(&Role::Admin) {
            match ctx.tokens.remaining[0].as_str() {
                "player" => {
                    if let Some(target) = ctx
                        .world
                        .players
                        .iter()
                        .find(|p| p.name == ctx.tokens.remaining[1].as_str())
                    {
                        return Ok(Response::Client(format!("{:#?}", target)));
                    }

                    Ok(Response::Client("Player not found.".to_string()))
                }
                "room" => {
                    if let Some(room) = ctx
                        .world
                        .rooms
                        .iter()
                        .find(|r| r.position == player.position)
                    {
                        return Ok(Response::Client(format!("{:#?}", room)));
                    }

                    Ok(Response::Client("Room not found.".to_string()))
                }
                "world" => Ok(Response::Client(format!("{}", ctx.world))),
                "connections" => Ok(Response::Client(format!(
                    "{:#?}",
                    ctx.world
                        .players
                        .iter()
                        .map(|p| format!("[{}] {}", p.id, p.name))
                        .collect::<Vec<_>>()
                        .join(", ")
                ))),
                "systems" => match ctx.tokens.remaining[1].as_str() {
                    "start" => {
                        let result = ctx
                            .world
                            .systems
                            .set_status(ctx.tokens.remaining[2].as_str(), SystemStatus::Running);

                        match result {
                            true => Ok(Response::Client(format!(
                                "Starting system `{}`.",
                                ctx.tokens.remaining[2].as_str()
                            ))),
                            false => Ok(Response::Client(format!(
                                "Could not find a system named `{}`.",
                                ctx.tokens.remaining[2].as_str()
                            ))),
                        }
                    }
                    "stop" => {
                        let result = ctx
                            .world
                            .systems
                            .set_status(ctx.tokens.remaining[2].as_str(), SystemStatus::Stopped);

                        match result {
                            true => Ok(Response::Client(format!(
                                "Stopping system `{}`.",
                                ctx.tokens.remaining[2].as_str()
                            ))),
                            false => Ok(Response::Client(format!(
                                "Could not find a system named `{}`.",
                                ctx.tokens.remaining[2].as_str()
                            ))),
                        }
                    }
                    "pause" => {
                        let result = ctx
                            .world
                            .systems
                            .set_status(ctx.tokens.remaining[2].as_str(), SystemStatus::Paused);

                        match result {
                            true => Ok(Response::Client(format!(
                                "Pausing system `{}`.",
                                ctx.tokens.remaining[2].as_str()
                            ))),
                            false => Ok(Response::Client(format!(
                                "Could not find a system named `{}`.",
                                ctx.tokens.remaining[2].as_str()
                            ))),
                        }
                    }
                    "restart" => Ok(Response::Client("Not implemented.".to_string())),
                    _ => ctx.world.unknown(player.id),
                },
                _ => ctx.world.unknown(player.id),
            }
        } else {
            ctx.world.unknown(player.id)
        }
    }
}
