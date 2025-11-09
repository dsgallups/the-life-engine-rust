mod state;
pub use state::*;

mod scheduler;
pub use scheduler::*;

use rand::rngs::StdRng;

use crate::prelude::JunctionAffer;
pub struct Brain {
    name: String,
    rng: StdRng,
    afference: Vec<JunctionAffer>,
}

impl Brain {
    /// This tick will queue up things
    pub fn tick(&mut self) {}
}
