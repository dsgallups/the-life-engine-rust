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

#[derive(Component)]
pub struct ActiveCell;

fn on_click_brain_cell(
    ev: On<Pointer<Click>>,
    mut commands: Commands,
    brain_cells: Query<&BrainCell>,
    active_cells: Query<Entity, With<ActiveCell>>,
) {
    let entity = ev.entity;
    let Ok(brain_cell) = brain_cells.get(entity) else {
        info!("No brain cell!");
        return;
    };
    for active_cell in active_cells {
        commands.entity(active_cell).remove::<ActiveCell>();
    }

    commands.entity(entity).insert(ActiveCell);

    info!("click!");
}
