/// Systems are ways to add a function into the core game loop. Each system runs once per server
/// 'tick', and runs in the order they were added. Systems do not stop, and as they are run often,
/// they should not block the thread for a long time. The game loop runs on a single thread, so it
/// would severely impact performance. Currently there is no way to spawn a task onto a new thread
/// from the game loop, but this is on the list of things to do.
///
/// There are currently two system types: `System` and `SystemReadOnly`. `System`'s have exclusive
/// references to the world and its parent struct, while `SystemReadOnly`'s do not. An important
/// note is that `SystemReadOnly`'s run after `System`'s. This means changes to state are reflected
/// immediately by those systems.
///
/// Systems require a name and a struct to be attached to; which could exist simply as a marker
/// with no data -- but if you wish to store your own state, this struct is where you would do so.
///
/// Systems are still a work-in-progress, so the API may change as I try to simplify usage while
/// increasing the extendability.
use crate::world::World;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemStatus {
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchStatus {
    Manual,
    Automatic,
}

pub trait System: Send + Sync {
    fn update(&mut self, world: &mut World);
}

/// `SystemHandle`'s should not be worked with directly; use `System` instead.
pub struct SystemHandle {
    pub status: SystemStatus,
    pub watch: WatchStatus,
    pub name: &'static str,
    pub inner: Box<dyn System>,
}

/// Represents a system that has exclusive, mutable access to the world and its parent struct.
impl SystemHandle {
    pub fn new(name: &'static str, inner: Box<dyn System>) -> Self {
        Self {
            status: SystemStatus::Running,
            watch: WatchStatus::Automatic,
            name,
            inner,
        }
    }
}

impl std::fmt::Display for SystemHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Represents a system that has shared, immutable access to the world and its parent struct.
pub trait SystemReadOnly: Send + Sync {
    fn update(&self, world: &World);
}

/// `SystemReadOnlyHandle`'s should not be worked with directly; use `SystemReadOnly` instead.
pub struct SystemReadOnlyHandle {
    pub status: SystemStatus,
    pub watch: WatchStatus,
    pub name: &'static str,
    pub inner: Box<dyn SystemReadOnly>,
}

impl SystemReadOnlyHandle {
    pub fn new(name: &'static str, inner: Box<dyn SystemReadOnly>) -> Self {
        Self {
            status: SystemStatus::Running,
            watch: WatchStatus::Automatic,
            name,
            inner,
        }
    }
}

impl std::fmt::Display for SystemReadOnlyHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (readonly)", self.name)
    }
}
