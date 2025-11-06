mod genome;

use bevy::{color::palettes::tailwind::BLUE_400, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(genome::plugin);

    app.add_systems(Startup, spawn_first_organism);
}

#[derive(Component, Reflect)]
#[relationship_target(relationship = CellOf)]
pub struct Cells(Vec<Entity>);

#[derive(Component, Reflect)]
#[relationship(relationship_target = Cells)]
pub struct CellOf(pub Entity);

fn spawn_first_organism(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let square = meshes.add(Rectangle::default());
    let white = materials.add(Color::WHITE);

    let organism = commands
        .spawn((InheritedVisibility::VISIBLE, Transform::default()))
        .id();

    commands.spawn((
        Mesh2d(square.clone()),
        MeshMaterial2d(white.clone()),
        Transform::from_xyz(1., 1., 0.),
        CellOf(organism),
    ));

    commands.spawn((
        Mesh2d(square.clone()),
        MeshMaterial2d(white.clone()),
        Transform::default(),
        CellOf(organism),
    ));

    commands.spawn((
        Mesh2d(square.clone()),
        MeshMaterial2d(white.clone()),
        Transform::from_xyz(-1., -1., 0.),
        CellOf(organism),
    ));
}
