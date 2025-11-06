use crate::game::{
    cell::{Collagen, DataCell, Launcher},
    organism::{
        genome::{CellType, Genome},
        CellOf,
    },
};
use bevy::{
    color::palettes::tailwind::{PINK_400, RED_600, YELLOW_400},
    prelude::*,
};

#[derive(Resource)]
struct CellAssets {
    cell: Handle<Mesh>,
    white: Handle<ColorMaterial>,
    pink: Handle<ColorMaterial>,
    red: Handle<ColorMaterial>,
    yellow: Handle<ColorMaterial>,
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
                InheritedVisibility::VISIBLE,
                msg.genome.clone(),
                Transform::from_xyz(msg.location.x, msg.location.y, 0.),
            ))
            .id();

        commands.spawn((
            ChildOf(organism),
            CellOf(organism),
            Mesh2d(assets.cell.clone()),
            MeshMaterial2d(assets.pink.clone()),
        ));

        for cell in msg.genome.cells() {
            let location = cell.location();
            let mut commands = commands.spawn((
                ChildOf(organism),
                CellOf(organism),
                Transform::from_xyz(location.x as f32, location.y as f32, 0.),
                Mesh2d(assets.cell.clone()),
            ));
            match cell.kind() {
                CellType::Collagen => {
                    commands.insert((Collagen::default(), MeshMaterial2d(assets.white.clone())));
                }
                CellType::Data => {
                    commands.insert((DataCell::default(), MeshMaterial2d(assets.yellow.clone())));
                }
                CellType::Launcher => {
                    commands.insert((Launcher::default(), MeshMaterial2d(assets.red.clone())));
                }
            }
        }
    }
}
