use serde::Deserialize;

use crate::{
    entity::{Entity, EntityId},
    quickmap::QuickMapKey,
    vec3::Vec3,
    world::World,
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
    id: EntityId,
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
    pub fn new(world: &mut World, template: MonsterTemplate) -> Self {
        Self {
            id: world.next_id(),
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
