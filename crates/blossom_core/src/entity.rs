#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: u32,
    pub tick: u64,
}

impl Entity {
    pub fn new(id: u32, tick: u64) -> Entity {
        Entity { id, tick }
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.id, self.tick)
    }
}
