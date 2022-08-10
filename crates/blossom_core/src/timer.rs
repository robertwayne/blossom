use std::{fmt::Display, time::Instant};

/// Used as the internal representation of the game loop time. This is updated
/// on each 'tick' of the game loop. This does NOT relate to in-game game time,
/// which is handled by its own separate timer, `GameTime`.
#[derive(Debug)]
pub struct Timer {
    // Represents the time at which the game loop started.
    pub start_time: Instant,
    // Reference to the tick_rate defined in the config file.
    pub tick_rate: u64,
    // The interval the game loop should wait before the next tick: 1000 /
    // tick_rate.
    pub interval: u64,
    // How many seconds have passed since the game loop started.
    pub seconds: u64,
    // How many ticks have passed since the game loop started.
    pub count: u64,
    // How long since the last non-system was executed on the game loop (eg.
    // commands, joined, etc.)
    pub last_action: u64,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            tick_rate: 20,
            interval: 1000 / 20,
            seconds: 0,
            count: 0,
            last_action: 0,
        }
    }

    /// Returns the game uptime in human-readable format (HH:MM:SS).
    pub fn uptime(&self) -> String {
        let hours = self.seconds / 3600;
        let minutes = (self.seconds % 3600) / 60;
        let seconds = self.seconds % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({} ticks @ {}tps ({}ms per tick))",
            self.uptime(),
            self.count,
            self.tick_rate,
            self.interval
        )
    }
}
