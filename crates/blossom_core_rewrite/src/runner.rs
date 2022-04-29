use bevy_app::prelude::*;

use crate::{systems::execution_timer::ExecutionTimer, timer::Timer};

pub fn runner(mut app: App) {
    // let mut timer = Timer::new();
    let mut et = ExecutionTimer::new();

    loop {
        let start = std::time::Instant::now();

        // Run ECS schedule (stages, systems)
        app.update();

        et.update(start);

        let mut timer = app.world.get_resource_mut::<Timer>().unwrap();
        timer.tick();
    }
}
