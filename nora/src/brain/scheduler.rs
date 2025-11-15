use crate::{brain::BrainState, prelude::JunctionAffer};

//todo
pub struct TickScheduler<'a> {
    state: &'a BrainState,
}
impl<'a> TickScheduler<'a> {
    pub fn init(state: &'a BrainState) -> Self {
        Self { state }
    }
}
