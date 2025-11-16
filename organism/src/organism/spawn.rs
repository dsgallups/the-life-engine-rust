use bevy::prelude::*;

use crate::{
    CellAssets, CellKind, CellOf, Collagen, DataCell, Eye, Launcher, Organism, genome::Genome,
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
        let mut organism_genome = msg.genome.deep_clone();
        organism_genome.scramble(&mut rand::rng());
        let organism = commands
            .spawn((
                Name::new("Organism"),
                Organism::new(msg.genome.clone()),
                InheritedVisibility::VISIBLE,
                Pickable::default(),
                Transform::from_xyz(msg.location.x, msg.location.y, 0.),
            ))
            .id();

        for (location, cell) in msg.genome.cells().map() {
            let mut commands = commands.spawn((
                cell.kind,
                ChildOf(organism),
                CellOf(organism),
                Pickable::default(),
                Transform::from_xyz(location.x as f32, location.y as f32, 0.),
                Mesh2d(assets.cell.clone()),
            ));
            match cell.kind {
                CellKind::Collagen => {
                    commands.insert((
                        Name::new("Collagen"),
                        Collagen::default(),
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
