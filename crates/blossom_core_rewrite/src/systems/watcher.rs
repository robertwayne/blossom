use bevy_ecs::prelude::*;

use crate::{
    player::Player,
    stores::system_store::SystemStore,
    system::{System, SystemStatus, WatchStatus},
    timer::Timer,
};

/// Represents what the watcher is looking for.
/// - `Wake`: The watcher will wake up systems when a player joins the server.
/// - `Sleep`: The watcher will put systems to sleep when there have been no actions for 5 minutes.
#[derive(Debug, PartialEq, Eq)]
enum WatchMode {
    Wake,
    Sleep,
}

/// This is an internal, core system that handles automatic system status updates (eg. outside of a
/// call to `.set_status()`.) In general, the system watcher will put systems to sleep if there have
/// been no connections for 5 minutes. As soon as a player joins the server, the system watcher will
/// wake up all the paused systems.
///
/// The goal is to save bandwitch and CPU cycles, however little, when the server is otherwise in a
/// dormant state.
///
/// In cases where you do not want a system to be managed by the watcher, you can set that systems
/// watch status to `Watch::Manual`.
#[derive(Debug)]
pub struct SystemWatcher {
    // See `WatchMode` for more information.
    mode: WatchMode,
    // How long to wait before putting systems to sleep.
    pub interval: u64,
}

impl SystemWatcher {
    pub fn new() -> Self {
        Self {
            mode: WatchMode::Sleep,
            interval: 5,
        }
    }
}

impl Default for SystemWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemWatcher {
    pub fn update(&mut self, systems: &mut SystemStore, timer: &Timer, players: usize) {
        if self.mode == WatchMode::Wake && players != 0 {
            tracing::debug!("New connection detected. Waking up all paused systems.");

            // Now we wait to put systems to sleep.
            self.mode = WatchMode::Sleep;

            for system in systems.write.iter_mut() {
                if system.status == SystemStatus::Paused && system.watch == WatchStatus::Automatic {
                    system.status = SystemStatus::Running;
                }
            }

            for system in systems.readonly.iter_mut() {
                if system.status == SystemStatus::Paused && system.watch == WatchStatus::Automatic {
                    system.status = SystemStatus::Running;
                }
            }
        } else if self.mode == WatchMode::Sleep
            && players == 0
            && timer.last_action + self.interval < timer.seconds
        {
            tracing::debug!(
                "No players connected for {} seconds. Suspending all systems.",
                self.interval
            );

            // Now we wait to wake back up.
            self.mode = WatchMode::Wake;

            for system in systems.write.iter_mut() {
                if system.status == SystemStatus::Running && system.watch == WatchStatus::Automatic
                {
                    system.status = SystemStatus::Paused;
                }
            }

            for system in systems.readonly.iter_mut() {
                if system.status == SystemStatus::Running && system.watch == WatchStatus::Automatic
                {
                    system.status = SystemStatus::Paused;
                }
            }
        }
    }
}
