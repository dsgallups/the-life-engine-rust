use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct GameCamera;

pub fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(5.0);
    commands.spawn((camera_bundle, GameCamera));
}

pub fn update_camera(
    //mouse:
    mut camera: Query<(&mut OrthographicProjection, &mut GameCamera)>,
) {
}
