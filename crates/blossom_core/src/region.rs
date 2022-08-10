use serde::Deserialize;

use crate::entity::{Entity, EntityId};

#[derive(Debug)]
pub struct Area {
    pub entity_id: EntityId,
    pub name: String,
    pub description: String,
    pub rooms: Vec<EntityId>,
    pub mob_pool: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AreaBuilder {
    pub region: String,
    pub name: String,
    pub description: String,
    pub mob_pool: Vec<String>,
}

impl AreaBuilder {
    pub fn build(self, id: EntityId) -> Area {
        Area {
            entity_id: id,
            name: self.name,
            description: self.description,
            rooms: Vec::new(),
            mob_pool: self.mob_pool,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegionBuilder {
    pub region: String,
    pub name: String,
    pub description: String,
    pub mob_pool: Vec<String>,
}

impl RegionBuilder {
    pub fn build(self, id: EntityId) -> Region {
        Region {
            entity_id: id,
            name: self.name,
            description: self.description,
            areas: Vec::new(),
            mob_pool: self.mob_pool,
        }
    }
}

#[derive(Debug)]
pub struct Region {
    pub entity_id: EntityId,
    pub name: String,
    pub description: String,
    pub areas: Vec<EntityId>,
    pub mob_pool: Vec<String>,
}

// impl QuickMapKey<String> for Region { fn key(&self) -> String { self.name } }

// impl QuickMapKey<String> for Area { fn key(&self) -> String { self.name } }

impl Entity for Region {
    fn id(&self) -> EntityId {
        self.entity_id
    }
}

impl Entity for Area {
    fn id(&self) -> EntityId {
        self.entity_id
    }
}
