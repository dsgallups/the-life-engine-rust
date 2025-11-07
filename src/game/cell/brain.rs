use bevy::prelude::*;
use nora_neat::{naive_net::network::SimpleNetwork, prelude::NetworkTopology};

pub(super) fn plugin(app: &mut App) {
    //todo
}

#[derive(Component)]
pub struct BrainCell {
    topology: NetworkTopology,
    network: SimpleNetwork,
}
impl BrainCell {
    pub fn new(topology: NetworkTopology) -> Self {
        Self {
            network: SimpleNetwork::from_topology(&topology),
            topology,
        }
    }
}
