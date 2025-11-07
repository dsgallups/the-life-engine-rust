use bevy::prelude::*;
use nora_neat::{naive_net::network::NaiveNetwork, prelude::NetworkTopology};

pub(super) fn plugin(app: &mut App) {
    //todo
}

#[derive(Component)]
pub struct BrainCell {
    topology: NetworkTopology,
    network: NaiveNetwork,
}
impl BrainCell {
    pub fn new(topology: NetworkTopology) -> Self {
        Self {
            network: NaiveNetwork::from_topology(&topology),
            topology,
        }
    }
}
