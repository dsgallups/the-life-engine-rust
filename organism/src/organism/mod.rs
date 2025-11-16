use crate::{
    CellAssets,
    CellDetails,
    CellOf,
    cell::{Collagen, DataCell, Eye, Launcher},
    //old_genome::Genome,
};
use bevy::prelude::*;
use nora_neat::prelude::NetworkTopology;

#[derive(Component)]
pub struct OrganismNetwork {
    topology: NetworkTopology,
}

impl OrganismNetwork {
    pub fn new(topology: NetworkTopology) -> Self {
        Self { topology }
    }
}

#[derive(Message)]
pub struct SpawnOrganism {
    //genome: Genome,
    location: Vec2,
}
impl SpawnOrganism {
    pub fn new<T>(genome: T, location: Vec2) -> Self {
        //Self { genome, location }
        Self { location }
    }
}

pub fn plugin(app: &mut App) {
    app.add_message::<SpawnOrganism>();
    app.add_systems(Update, spawn_genomes);
}

fn spawn_genomes(
    mut msgs: MessageReader<SpawnOrganism>,
    mut commands: Commands,
    assets: Res<CellAssets>,
) {
    // for msg in msgs.read() {
    //     let organism = commands
    //         .spawn((
    //             Name::new("Organism"),
    //             OrganismNetwork::new(msg.genome.network().deep_clone()),
    //             InheritedVisibility::VISIBLE,
    //             msg.genome.clone(),
    //             Pickable::default(),
    //             Transform::from_xyz(msg.location.x, msg.location.y, 0.),
    //         ))
    //         .id();

    //     for cell in msg.genome.cells() {
    //         let location = cell.location();
    //         let mut commands = commands.spawn((
    //             cell.details().cell_type(),
    //             ChildOf(organism),
    //             CellOf(organism),
    //             Pickable::default(),
    //             Transform::from_xyz(location.x as f32, location.y as f32, 0.),
    //             Mesh2d(assets.cell.clone()),
    //         ));
    //         match cell.details() {
    //             CellDetails::Collagen => {
    //                 commands.insert((
    //                     Name::new("Collagen"),
    //                     Collagen::default(),
    //                     MeshMaterial2d(assets.white.clone()),
    //                 ));
    //             }
    //             CellDetails::Data => {
    //                 commands.insert((
    //                     Name::new("Data Cell"),
    //                     DataCell::default(),
    //                     MeshMaterial2d(assets.yellow.clone()),
    //                 ));
    //             }
    //             CellDetails::Launcher => {
    //                 commands.insert((
    //                     Name::new("Launcher Cell"),
    //                     Launcher::default(),
    //                     MeshMaterial2d(assets.red.clone()),
    //                 ));
    //             }
    //             CellDetails::Eye => {
    //                 commands.insert((
    //                     Name::new("Eye Cell"),
    //                     Eye::default(),
    //                     MeshMaterial2d(assets.sky.clone()),
    //                 ));
    //             }

    //             CellDetails::Brain => {
    //                 // todo
    //                 todo!()
    //             } // CellDetails::Brain(topology) => {
    //               //     commands.insert((
    //               //         Name::new("Brain Cell"),
    //               //         BrainCell::new(topology.deep_clone()),
    //               //         MeshMaterial2d(assets.pink.clone()),
    //               //     ));
    //               // }
    //         }
    //     }
    // }
}
