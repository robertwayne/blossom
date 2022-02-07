use crate::{player::PlayerId, token_stream::TokenStream, world::World};

pub struct Context<'a> {
    pub id: PlayerId,
    pub tokens: TokenStream,
    pub world: &'a mut World,
}

impl<'a> Context<'a> {
    pub fn new(id: PlayerId, tokens: TokenStream, world: &'a mut World) -> Self {
        Self { id, tokens, world }
    }
}
