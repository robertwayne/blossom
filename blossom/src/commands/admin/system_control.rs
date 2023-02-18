use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::Result,
    response::Response,
    role::Role,
    system::SystemStatus,
    world::World,
};

pub struct SystemsControl;

impl GameCommand for SystemsControl {
    fn create() -> Command {
        Command {
            name: "@systems",
            description: "Information about game systems.",
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let player = ctx.world.get_player(ctx.id)?;

        if player.account.roles.contains(&Role::Admin) {
            let mut tokens = ctx.input.args.iter();

            match tokens.next() {
                Some(token) => match token.as_str() {
                    "start" => {
                        if let Some(command) = ctx.input.args.get(1) {
                            let result = ctx
                                .world
                                .systems
                                .set_status(command.as_str(), SystemStatus::Running);

                        if result {
                             Ok(Response::Client(format!(
                                "Starting system `{command}`.",
                            )))
                         }else { Ok(Response::Client(format!(
                                "Could not find a system named `{command}`.",
                            )))
                        }
                        } else {
                            Ok(Response::Client(format!("No system named {token}", )))
                        }
                    }
                    "stop" => {
                        if let Some(command) = ctx.input.args.get(1) {
                            let result = ctx
                                .world
                                .systems
                                .set_status(command.as_str(), SystemStatus::Stopped);

                            if result { Ok(Response::Client(format!(
                                    "Stopping system `{command}`."
                                )))
                             } else { Ok(Response::Client(format!(
                                    "Could not find a system named `{command}`."
                                )))
                            }
                        } else {
                            Ok(Response::Client(format!("No system named {token}")))
                        }
                    }
                    "pause" => {
                        if let Some(command) = ctx.input.args.get(1) {
                            let result = ctx
                                .world
                                .systems
                                .set_status(command.as_str(), SystemStatus::Paused);

                            if result { Ok(Response::Client(format!(
                                    "Pausing system `{command}`."
                                )))
                            } else {
                                Ok(Response::Client(format!(
                                    "Could not find a system named `{command}`."
                                )))
                            }
                        } else {
                            Ok(Response::Client(format!("No system named {token}")))
                        }
                    }
                    "restart" => Ok(Response::Client("Not implemented.".to_string())),
                    _ => Ok(Response::Client("Invalid system command. Options are ['start', 'stop', 'pause', 'restart'].".to_string())),
                },
                None => Ok(Response::Client("Various system control commands.".to_string())),
            }
        } else {
            World::unknown(player.id)
        }
    }
}
