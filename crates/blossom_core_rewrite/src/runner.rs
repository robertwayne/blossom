use bevy_app::prelude::*;

use crate::{
    stores::system_store::SystemStore, system::SystemStatus,
    systems::execution_timer::ExecutionTimer, timer::Timer,
};

pub fn runner(mut app: App) {
    let mut timer = Timer::new();
    let mut et = ExecutionTimer::new();

    loop {
        let start = std::time::Instant::now();

        // Run ECS schedule (stages, systems)
        app.update();

        et.update(start);
        timer.tick();
    }
}
