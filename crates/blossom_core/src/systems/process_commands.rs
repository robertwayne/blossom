use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use tracing::debug;

use crate::{
    event::{ClientEvent, Event},
    world::Connection,
};

pub fn process_commands(conn: Res<Connection>) {
    debug!("Processing commands");
    while let Ok(Event::Client(id, event)) = conn.rx.try_recv() {
        tracing::debug!("Received event: {:?}", event);
        match event {
            ClientEvent::Connect(player, _) => {
                debug!("Player connected: {:?}", player);
            }
            _ => {}
        }
    }
}
