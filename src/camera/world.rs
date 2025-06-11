use bevy::{color::palettes::css::BLACK, prelude::*, render::view::RenderLayers};

use crate::camera::{CameraOrder, RenderLayer};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_world_camera);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WorldCamera;

fn spawn_world_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("World Camera"),
        Camera2d,
        WorldCamera,
        Camera {
            order: CameraOrder::World.into(),
            clear_color: ClearColorConfig::Custom(BLACK.into()),
            ..default()
        },
        Transform::from_xyz(0., 0., 0.)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_scale(Vec3::splat(0.1)),
        RenderLayers::from(RenderLayer::DEFAULT | RenderLayer::PARTICLES | RenderLayer::GIZMO3),
    ));
}
