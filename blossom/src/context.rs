use crate::{input::Input, player::PlayerId, world::World};

pub struct Context<'a> {
    pub id: PlayerId,
    pub input: Input,
    pub world: &'a mut World,
}

impl<'a> Context<'a> {
    pub fn new(id: PlayerId, tokens: Input, world: &'a mut World) -> Self {
        Self {
            id,
            input: tokens,
            world,
        }
    }

    pub fn args(&self) -> &[String] {
        self.input.args.as_slice()
    }
}

impl std::fmt::Display for Context<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Context(id={}, tokens={:?})", self.id, self.input)
    }
}
