use iridescent::Styled;

use crate::{
    system::{SystemHandle, SystemReadOnlyHandle, SystemStatus, WatchStatus},
    systems::{execution_timer::ExecutionTimer, watcher::SystemWatcher},
    theme,
};

pub struct SystemStore {
    pub write: Vec<SystemHandle>,
    pub readonly: Vec<SystemReadOnlyHandle>,
    pub execution_timer: ExecutionTimer,
    pub watcher: SystemWatcher,
}

impl SystemStore {
    pub fn new() -> Self {
        Self {
            write: Vec::new(),
            readonly: Vec::new(),
            execution_timer: ExecutionTimer::new(),
            watcher: SystemWatcher::new(),
        }
    }

    pub fn set_status(&mut self, system_name: &str, status: SystemStatus) -> bool {
        let mut result = false;

        for system in &mut self.write {
            if system.name == system_name {
                system.status = status;
                system.watch = WatchStatus::Manual;
                result = true;
            }
        }

        for system in &mut self.readonly {
            if system.name == system_name {
                system.status = status;
                system.watch = WatchStatus::Manual;
                result = true;
            }
        }

        result
    }
}

impl Default for SystemStore {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SystemStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s1 = self.write.iter().map(|s| (s.name, s.status)).collect::<Vec<_>>();
        let s2 = self.readonly.iter().map(|s| (s.name, s.status)).collect::<Vec<_>>();

        let system_set = s1.iter().chain(s2.iter()).copied().collect::<Vec<_>>();

        let text = format!(
            "{} ({}, {} readonly)\n  Running: [{}]\n  Paused: [{}]\n  Stopped: [{}]",
            system_set.len().to_string().foreground(theme::GREEN).bold(),
            self.write.len().to_string().foreground(theme::GREEN).bold(),
            self.readonly.len().to_string().foreground(theme::GREEN).bold(),
            system_set
                .iter()
                .filter(|s| s.1 == SystemStatus::Running)
                .map(|s| format!("{}", s.0.to_string().foreground(theme::GREEN).bold()))
                .collect::<Vec<_>>()
                .join(", "),
            system_set
                .iter()
                .filter(|s| s.1 == SystemStatus::Paused)
                .map(|s| format!("{}", s.0.to_string().foreground(theme::YELLOW).bold()))
                .collect::<Vec<_>>()
                .join(", "),
            system_set
                .iter()
                .filter(|s| s.1 == SystemStatus::Stopped)
                .map(|s| format!("{}", s.0.to_string().foreground(theme::RED).bold()))
                .collect::<Vec<_>>()
                .join(", "),
        );

        write!(f, "{text}",)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        system::{System, WatchStatus},
        world::World,
    };

    #[test]
    fn change_status() {
        struct MockSystem;

        impl System for MockSystem {
            fn update(&mut self, _: &mut World) {}
        }

        let mut system_store = SystemStore::new();

        system_store.write.push(SystemHandle {
            inner: Box::new(MockSystem),
            name: "test",
            status: SystemStatus::Running,
            watch: WatchStatus::Automatic,
        });

        assert!(system_store.set_status("test", SystemStatus::Running));
        assert!(system_store.set_status("test", SystemStatus::Paused));
        assert!(system_store.set_status("test", SystemStatus::Stopped));
        assert!(!system_store.set_status("non_existent_test", SystemStatus::Paused));
    }
}
