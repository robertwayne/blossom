use iridescent::Styled;
use serde::Deserialize;

use crate::{
    entity::{Entity, EntityId},
    quickmap::QuickMapKey,
    searchable::Searchable,
    vec3::Vec3,
};

#[derive(Debug, Deserialize, Clone)]
pub struct MonsterTemplate {
    name: String,
    description: String,
    health: i32,
}

impl MonsterTemplate {
    pub fn create_key(&self) -> String {
        self.name.to_lowercase().replace(' ', "_")
    }
}

#[derive(Debug)]
pub struct Monster {
    pub id: EntityId,
    pub name: String,
    pub description: String,
    pub position: Vec3,
    pub health: i32,
    pub max_health: i32,
}

impl Monster {
    pub fn with_position(&mut self, position: Vec3) -> &mut Self {
        self.position = position;
        self
    }
}

impl Monster {
    pub fn new(id: EntityId, template: MonsterTemplate) -> Self {
        Self {
            id,
            name: template.name,
            description: template.description,
            position: Vec3::default(),
            health: template.health,
            max_health: template.health,
        }
    }
}

impl Entity for Monster {
    fn id(&self) -> EntityId {
        self.id
    }
}

impl QuickMapKey<EntityId> for Monster {
    fn key(&self) -> EntityId {
        self.id
    }
}

impl<'a> Searchable for &'a Monster {
    fn search_key(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for Monster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n{}", self.name.bold(), self.description)
    }
}
