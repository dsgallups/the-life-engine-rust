use avian2d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::prelude::*;

use crate::{
    cell::{CellAssets, CellKind, CellOf, DataCell, Eye, Foot, Launcher},
    cpu_net::CpuNetwork,
    genome::Genome,
    organism::Organism,
};

#[derive(Message)]
pub struct SpawnOrganism {
    genome: Genome,
    location: Vec2,
}
impl SpawnOrganism {
    /// The receiving system will create offspring from this genome.
    pub fn new(genome: Genome, location: Vec2) -> Self {
        Self { genome, location }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_message::<SpawnOrganism>();
    app.add_systems(Update, spawn_genomes);
}

fn spawn_genomes(
    mut msgs: MessageReader<SpawnOrganism>,
    mut commands: Commands,
    assets: Res<CellAssets>,
) {
    for msg in msgs.read() {
        let organism = commands
            .spawn((
                Name::new("Organism"),
                Organism::new(msg.genome.clone()),
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                InheritedVisibility::VISIBLE,
                Pickable::default(),
                Transform::from_xyz(msg.location.x, msg.location.y, 0.),
            ))
            .observe(super::ui::set_active)
            .id();

        let cpu_net = CpuNetwork::new(&msg.genome);

        for (location, cell) in cpu_net.cells {
            let kind = cell.kind();
            let mut commands = commands.spawn((
                cell,
                ChildOf(organism),
                CellOf(organism),
                Collider::rectangle(1., 1.),
                Pickable::default(),
                Transform::from_xyz(location.x as f32, location.y as f32, 0.),
                Mesh2d(assets.cell.clone()),
            ));

            match kind {
                CellKind::Foot => {
                    commands.insert((
                        Name::new("Collagen"),
                        Foot::default(),
                        MeshMaterial2d(assets.white.clone()),
                    ));
                }
                CellKind::Data => {
                    commands.insert((
                        Name::new("Data Cell"),
                        DataCell::default(),
                        MeshMaterial2d(assets.yellow.clone()),
                    ));
                }
                CellKind::Launcher => {
                    commands.insert((
                        Name::new("Launcher Cell"),
                        Launcher::default(),
                        MeshMaterial2d(assets.red.clone()),
                    ));
                }
                CellKind::Eye => {
                    commands.insert((
                        Name::new("Eye Cell"),
                        Eye::default(),
                        MeshMaterial2d(assets.sky.clone()),
                    ));
                }
            }
        }
    }
}
