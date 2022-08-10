use std::time::Instant;

/// Internal, core system for tracking the execution time of game ticks. This means the time it
/// takes for all of the functions inside the game loop to run, BEFORE calling `.tick()`. This
/// is useful for debugging and profiling overall game performance -- you can think of it as your
/// server frame time, even though iterations are hard-capped by the `tick_rate` variable.
///
/// If somehow your tick execution time exceeded your tick interval (sleep duration), the game loop
/// would slowly drift off a consistent time, and could affect other systems signifigantly. In
/// practice, though, this would be almost impossible to do. The default 50ms per tick leaves an
/// massive amount of room, even for ineffecient systems or high command processing loads.
///
/// Note: This is a UNIQUE system. It does NOT operate on the system queue, and is baked into the
/// game loop. It will not show up under systems debug output, and cannot be dynamically reloaded.
#[derive(Debug)]
pub struct ExecutionTimer {
    // Tracks the last 100 ticks of the game loop.
    times: [u128; 100],
    // Tracks iterations of the game loop. Needed to calculate the index for the next time.
    count: u8,
}

impl ExecutionTimer {
    pub fn new() -> Self {
        Self {
            times: [0; 100],
            count: 0,
        }
    }

    /// Returns the average time for a tick, or otherwise all the functions within the game loop,
    /// to be executed. Sampled over the last 100 iterations. This will need at least a few
    /// seconds to be accurate.
    fn average_tick_execution_time(&self) -> u128 {
        let mut sum = 0;

        for i in 0..self.times.len() {
            sum += self.times[i];
        }

        sum / self.times.len() as u128
    }

    /// Returns a human-readable string of the average execution time for a tick converted to the
    /// nearest time magnitude.
    pub fn average(&self) -> String {
        let time = self.average_tick_execution_time();

        if time < 1000 {
            format!("{}ns", time)
        } else if time < 1_000_000 {
            format!("{}µs", time / 1000)
        } else if time < 1_000_000_000 {
            format!("{}ms", time / 1_000_000)
        } else {
            format!("{}s", time / 1_000_000_000)
        }
    }

    pub fn update(&mut self, start: Instant) {
        self.times[self.count as usize] = start.elapsed().as_nanos();
        self.count = (self.count + 1) % 100;

        if self.count == 1 {
            tracing::debug!("Average Execution Time: {}", self.average());
        }
    }
}

impl Default for ExecutionTimer {
    fn default() -> Self {
        Self::new()
    }
}
