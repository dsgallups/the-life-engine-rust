use bevy::prelude::*;
use nora_neat::{naive_net::network::NaiveNetwork, prelude::NetworkTopology};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_click_brain_cell);
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

fn on_click_brain_cell(
    ev: On<Pointer<Click>>,
    mut commands: Commands,
    brain_cells: Query<&BrainCell>,
) {
    let Ok(brain_cell) = brain_cells.get(ev.entity) else {
        info!("No brain cell!");
        return;
    };

    info!("click!");
}
