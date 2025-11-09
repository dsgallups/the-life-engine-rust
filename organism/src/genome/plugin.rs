use crate::{
    CellOf,
    cell::{BrainCell, Collagen, DataCell, Eye, Launcher},
    genome::{CellDetails, Genome},
};
use bevy::{
    color::palettes::tailwind::{PINK_400, RED_600, SKY_300, YELLOW_400},
    prelude::*,
};

#[derive(Resource)]
struct CellAssets {
    cell: Handle<Mesh>,
    white: Handle<ColorMaterial>,
    pink: Handle<ColorMaterial>,
    red: Handle<ColorMaterial>,
    yellow: Handle<ColorMaterial>,
    sky: Handle<ColorMaterial>,
}

impl FromWorld for CellAssets {
    fn from_world(world: &mut World) -> Self {
        let cell = world
            .resource_mut::<Assets<Mesh>>()
            .add(Rectangle::default());
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        Self {
            cell,
            white: materials.add(Color::WHITE),
            pink: materials.add(Color::from(PINK_400)),
            red: materials.add(Color::from(RED_600)),
            yellow: materials.add(Color::from(YELLOW_400)),
            sky: materials.add(Color::from(SKY_300)),
        }
    }
}

#[derive(Message)]
pub struct SpawnOrganism {
    genome: Genome,
    location: Vec2,
}
impl SpawnOrganism {
    pub fn new(genome: Genome, location: Vec2) -> Self {
        Self { genome, location }
    }
}

pub fn plugin(app: &mut App) {
    app.add_message::<SpawnOrganism>();
    app.init_resource::<CellAssets>();
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
                InheritedVisibility::VISIBLE,
                msg.genome.clone(),
                Pickable::default(),
                Transform::from_xyz(msg.location.x, msg.location.y, 0.),
            ))
            .id();

        for cell in msg.genome.cells() {
            let location = cell.location();
            let mut commands = commands.spawn((
                cell.details().cell_type(),
                ChildOf(organism),
                CellOf(organism),
                Pickable::default(),
                Transform::from_xyz(location.x as f32, location.y as f32, 0.),
                Mesh2d(assets.cell.clone()),
            ));
            match cell.details() {
                CellDetails::Collagen => {
                    commands.insert((
                        Name::new("Collagen"),
                        Collagen::default(),
                        MeshMaterial2d(assets.white.clone()),
                    ));
                }
                CellDetails::Data => {
                    commands.insert((
                        Name::new("Data Cell"),
                        DataCell::default(),
                        MeshMaterial2d(assets.yellow.clone()),
                    ));
                }
                CellDetails::Launcher => {
                    commands.insert((
                        Name::new("Launcher Cell"),
                        Launcher::default(),
                        MeshMaterial2d(assets.red.clone()),
                    ));
                }
                CellDetails::Eye => {
                    commands.insert((
                        Name::new("Eye Cell"),
                        Eye::default(),
                        MeshMaterial2d(assets.sky.clone()),
                    ));
                }
                CellDetails::Brain(topology) => {
                    commands.insert((
                        Name::new("Brain Cell"),
                        BrainCell::new(topology.deep_clone()),
                        MeshMaterial2d(assets.pink.clone()),
                    ));
                }
            }
        }
    }
}
