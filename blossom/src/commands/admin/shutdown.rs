use std::{thread::sleep, time::Duration};

use crate::{
    command::{Command, GameCommand},
    context::Context,
    error::{ErrorType, Result},
    event::{Event, GameEvent},
    prelude::Error,
    response::Response,
    role::Role,
};

pub struct Shutdown;

impl GameCommand for Shutdown {
    fn create() -> Command {
        Command {
            name: "@shutdown",
            description: "Shuts down the game after a 30 second countdown.",
            ..Default::default()
        }
    }

    fn run(ctx: Context) -> Result<Response> {
        let binding = ctx.world.players.read();
        let Some(player) = binding.get(&ctx.id) else {
            return Err(Error::new(ErrorType::Internal, "Player not found."));
        };

        if player.account.roles.contains(&Role::Admin) {
            let players = ctx
                .world
                .players
                .read()
                .iter()
                .map(|p| p.id)
                .collect::<Vec<_>>();

            ctx.world.send_command(
                player.id,
                Response::Channel(
                    players,
                    "Server shutting down in 30 seconds. Please log out to save your progress."
                        .to_string(),
                ),
            );

            let tx = ctx.world.broker.clone();
            let players = ctx
                .world
                .players
                .read()
                .iter()
                .filter(|p| p.dirty)
                .cloned()
                .collect();

            // can we return from a task?
            tokio::spawn(async move {
                sleep(Duration::from_secs(30));

                tracing::info!("Running global save...");
                let _ = tx
                    .send_async(Event::Game(
                        -1, // this ID doesn't actually matter
                        GameEvent::GlobalSave(players),
                    ))
                    .await;
                tracing::info!("Global save complete.");

                sleep(Duration::from_secs(10));

                std::process::exit(0);
            });
        }

        Ok(Response::Empty)
    }
}
