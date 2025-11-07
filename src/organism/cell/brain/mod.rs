mod ui;

mod visual;
pub use visual::*;

use bevy::prelude::*;
use nora_neat::{naive_net::network::NaiveNetwork, prelude::NetworkTopology};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ui::plugin, visual::plugin));
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
    pub fn network(&self) -> &NaiveNetwork {
        &self.network
    }
}
