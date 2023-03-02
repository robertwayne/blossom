use crate::{system::System, world::World};

pub struct Spawner {
    pub interval: u64,
    pub last_run: u64,
}

impl Spawner {
    pub fn new(interval: u64) -> Self {
        Self {
            interval,
            last_run: 0,
        }
    }
}

impl System for Spawner {
    fn update(&mut self, world: &mut World) {
        if self.last_run + self.interval < world.timer.seconds {
            self.last_run = world.timer.seconds;

            let _rng = rand::thread_rng();

            // @TODO: This needs to be replaced to avoid the mutable borrow
            // after an immutable borrow.
            // if let Some(room) = world.rooms.iter().choose(&mut rng) {
            //     if let Some(key) = room.mob_pool.iter().choose(&mut rng) {
            //         world.spawn_monster(&key.to_lowercase().replace(' ', "_"), room.position);
            //     }
            // }
        }
    }
}
