pub trait Entity {
    fn id(&self) -> EntityId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId {
    pub id: u32,
    pub tick: u64,
}

impl EntityId {
    pub fn empty() -> Self {
        EntityId { id: 0, tick: 0 }
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.id, self.tick)
    }
}
