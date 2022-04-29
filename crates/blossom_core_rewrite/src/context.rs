use crate::{input::Input, player::PlayerId};

pub struct Context {
    pub id: PlayerId,
    pub input: Input,
}

impl Context {
    pub fn new(id: PlayerId, tokens: Input) -> Self {
        Self { id, input: tokens }
    }

    pub fn args(&self) -> &[String] {
        self.input.args.as_slice()
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Context(id={}, tokens={:?})", self.id, self.input)
    }
}
