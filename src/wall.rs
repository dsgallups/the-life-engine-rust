use avian2d::prelude::*;
use bevy::{color::palettes::tailwind::GRAY_400, prelude::*};

use crate::food::Health;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_walls);
}

#[derive(Component, Reflect)]
pub struct Wall;

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const OFFSET: f32 = 100.;

    const WALL_WIDTH: f32 = 5.;
    const WALL_HEIGHT: f32 = OFFSET * 2. + WALL_WIDTH;
    let gray = materials.add(Color::from(GRAY_400));
    commands
        .spawn((
            Name::new("Left Wall"),
            Wall,
            RigidBody::Static,
            CollisionEventsEnabled,
            Collider::rectangle(WALL_WIDTH, WALL_HEIGHT),
            Mesh2d(meshes.add(Rectangle::new(WALL_WIDTH, WALL_HEIGHT))),
            MeshMaterial2d(gray.clone()),
            Transform::from_xyz(-OFFSET, 0., 0.),
        ))
        .observe(wall_collision);

    commands
        .spawn((
            Name::new("Right Wall"),
            Wall,
            RigidBody::Static,
            CollisionEventsEnabled,
            Collider::rectangle(WALL_WIDTH, WALL_HEIGHT),
            Mesh2d(meshes.add(Rectangle::new(WALL_WIDTH, WALL_HEIGHT))),
            MeshMaterial2d(gray.clone()),
            Transform::from_xyz(OFFSET, 0., 0.),
        ))
        .observe(wall_collision);

    commands
        .spawn((
            Name::new("Top Wall"),
            Wall,
            RigidBody::Static,
            CollisionEventsEnabled,
            Collider::rectangle(WALL_HEIGHT, WALL_WIDTH),
            Mesh2d(meshes.add(Rectangle::new(WALL_HEIGHT, WALL_WIDTH))),
            MeshMaterial2d(gray.clone()),
            Transform::from_xyz(0., OFFSET, 0.),
        ))
        .observe(wall_collision);

    commands
        .spawn((
            Name::new("Bottom Wall"),
            Wall,
            RigidBody::Static,
            CollisionEventsEnabled,
            Collider::rectangle(WALL_HEIGHT, WALL_WIDTH),
            Mesh2d(meshes.add(Rectangle::new(WALL_HEIGHT, WALL_WIDTH))),
            MeshMaterial2d(gray.clone()),
            Transform::from_xyz(0., -OFFSET, 0.),
        ))
        .observe(wall_collision);
}
fn wall_collision(ev: On<CollisionStart>, mut organisms: Query<(&mut Health)>) {
    if let Some(body) = ev.body2
        && let Ok(mut health) = organisms.get_mut(body)
    {
        health.0 = 0;
    };
}
