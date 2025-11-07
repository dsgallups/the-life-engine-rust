use bevy::prelude::*;
use nora_neat::{prelude::NetworkTopology, simple_net::network::SimpleNetwork};

pub(super) fn plugin(app: &mut App) {
    //todo
}

#[derive(Component)]
pub struct BrainCell {
    network: SimpleNetwork,
}
impl BrainCell {
    pub fn new(topology: &NetworkTopology) -> Self {
        Self {
            network: SimpleNetwork::from_topology(topology),
        }
    }
}
