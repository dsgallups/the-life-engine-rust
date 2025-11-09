use bus::{Bus, BusReader};

use crate::CellTemplate;

pub trait Junction {
    /// The network's side that receives input
    fn afference(&mut self) -> Option<Vec<JunctionAffer>>;
    /// the network will push output here
    fn efference(&mut self) -> Option<Vec<JunctionEffer>>;
}

/// This receives input
pub struct JunctionAffer {
    channel: BusReader<f32>,
    cur_value: f32,
}

/// This receives input
pub struct JunctionEffer {
    channel: Bus<f32>,
}

#[derive(Default)]
pub struct NetworkTopology {
    inputs: Vec<JunctionAffer>,
    outputs: Vec<JunctionEffer>,
}

impl NetworkTopology {
    pub fn insert_junction(&mut self, junction: &mut impl Junction) {
        if let Some(afferers) = junction.afference() {
            self.inputs.extend(afferers);
        }
        if let Some(efferers) = junction.efference() {
            self.outputs.extend(efferers);
        }
    }
}
