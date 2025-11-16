mod state;
pub use state::*;

mod scheduler;
pub use scheduler::*;

mod neuron;

use rand::rngs::StdRng;

use crate::prelude::JunctionAffer;
#[derive(Default)]
pub struct Brain {
    // name: String,
    // rng: StdRng,
    state: BrainState,
    afference: Vec<JunctionAffer>,
}

impl Brain {
    /// This tick will queue up things
    pub fn tick(&mut self) {}
}
