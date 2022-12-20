use std::collections::HashMap;

use iridescent::Styled;

use crate::{
    entity::EntityId,
    monster::{Monster, MonsterTemplate},
    quickmap::QuickMap,
    theme,
};

#[derive(Debug)]
pub struct MonsterStore {
    map: QuickMap<EntityId, Monster>,
    templates: HashMap<String, MonsterTemplate>,
}

impl MonsterStore {
    pub fn new() -> Self {
        Self { map: QuickMap::new(), templates: HashMap::new() }
    }

    pub fn insert(&mut self, monster: Monster) -> EntityId {
        self.map.insert(monster)
    }

    pub fn remove(&mut self, id: EntityId) {
        self.map.remove_by_id(id);
    }

    pub fn insert_template(&mut self, key: String, template: MonsterTemplate) {
        self.templates.insert(key, template);
    }

    pub fn remove_template(&mut self, key: &str) {
        self.templates.remove(key);
    }

    pub fn get_template(&self, key: &str) -> Option<&MonsterTemplate> {
        self.templates.get(key)
    }

    pub fn get(&self, id: EntityId) -> Option<&Monster> {
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Monster> {
        self.map.get_mut(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Monster> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Monster> {
        self.map.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl std::fmt::Display for MonsterStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monsters: {} actual, {} templates",
            self.map.len().to_string().foreground(theme::GREEN).bold(),
            self.templates.len().to_string().foreground(theme::YELLOW).bold()
        )
    }
}

impl Default for MonsterStore {
    fn default() -> Self {
        Self::new()
    }
}
