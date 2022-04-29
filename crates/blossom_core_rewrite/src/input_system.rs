use bevy_ecs::prelude::*;

use flume::Receiver;
use iridescent::{Styled, CYAN, YELLOW};

use crate::{
    commands::unknown::unknown_command,
    context::Context,
    event::{ClientEvent, Event, GameEvent},
    player::Player,
    prompt::Prompt,
    response::Response,
    server::{send_command, send_event, GameCommandMap, Peers},
    timer::Timer,
};

pub fn input_system(
    mut commands: bevy_ecs::system::Commands,
    players: Query<(Entity, &Player)>,
    rx: Res<Receiver<Event>>,
    mut timer: ResMut<Timer>,
    mut peers: ResMut<Peers>,
    mut game_commands: ResMut<GameCommandMap>,
) {
    if let Ok(event) = rx.try_recv() {
        if let Event::Client(id, e) = event {
            match e {
                ClientEvent::Connect(player, tx) => {
                    send_event(
                        id,
                        &tx,
                        GameEvent::Accepted(Response::Client(format!(
                            "\n{}{}{}\n",
                            "Welcome back, ".foreground(YELLOW),
                            player.name.foreground(CYAN),
                            "!".foreground(YELLOW)
                        ))),
                    );

                    peers.insert(player.id, tx);
                    commands.spawn().insert(player);
                }
                ClientEvent::Disconnect => {
                    peers.remove(&id);

                    for (entity, player) in players.iter() {
                        if player.id == id {
                            commands.entity(entity).despawn();
                            break;
                        }
                    }
                }
                ClientEvent::Ping => {
                    let tx = peers
                        .get(&id)
                        .expect("peer disconnected before receiving a response");

                    for (_, player) in players.iter() {
                        if player.id == id {
                            send_command(
                                id,
                                tx,
                                Response::Client(Prompt::from(player).to_string()),
                            );
                            break;
                        }
                    }
                }
                ClientEvent::Command(input) => {
                    let result = match game_commands.get_mut(&input.command) {
                        Some(handle) => (handle.func)(Context::new(id, input)),
                        None => unknown_command(id),
                    };

                    let tx = peers
                        .get(&id)
                        .expect("peer disconnected before receiving a response");

                    if let Ok(result) = result {
                        let _ = tx.send(Event::Game(id, GameEvent::Command(result)));
                    }

                    for (_, player) in players.iter() {
                        if player.id == id {
                            send_command(
                                id,
                                tx,
                                Response::Client(Prompt::from(player).to_string()),
                            );
                            break;
                        }
                    }
                }
            }
        }

        timer.update();
    }
}
