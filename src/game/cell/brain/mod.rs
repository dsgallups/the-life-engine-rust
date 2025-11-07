mod ui;

use bevy::prelude::*;
use nora_neat::{naive_net::network::NaiveNetwork, prelude::NetworkTopology};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ui::plugin));
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

#[derive(Component)]
pub struct ActiveCell;

#[derive(Component)]
pub struct CellVisual;
