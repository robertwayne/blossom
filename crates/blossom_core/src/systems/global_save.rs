// use crate::{event::GameEvent, player::Player, system::System, world::World};

// /// Internal, core system that handles saving the game state to the database on a regular interval.
// pub struct GlobalSave {
//     pub interval: u64,
//     pub last_run: u64,
// }

// impl GlobalSave {
//     pub fn new(interval: u64) -> Self {
//         Self {
//             interval,
//             last_run: 0,
//         }
//     }
// }

// impl System for GlobalSave {
//     fn update(&mut self, world: &mut World) {
//         if self.last_run + self.interval < world.timer.seconds {
//             self.last_run = world.timer.seconds;

//             let mut query = world._world.query::<&Player>();
//             let mut players: Vec<Player> = Vec::new();
//             query.iter(&world._world).for_each(|player| {
//                 players.push(player.clone());
//             });

//             world.send_event(
//                 -1, // this ID doesn't actually matter
//                 GameEvent::GlobalSave(players),
//             )
//         }
//     }
// }
