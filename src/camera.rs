use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
    render::camera::ScalingMode,
};

#[derive(Component)]
pub struct GameCamera;

pub fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(5.0);
    commands.spawn((camera_bundle, GameCamera));
}

pub fn update_camera(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_scroll: EventReader<MouseWheel>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<GameCamera>>,
) {
    let (mut transform, mut proj) = camera.get_single_mut().unwrap();

    for ev in mouse_scroll.read() {
        if ev.unit == MouseScrollUnit::Pixel {
            warn!("Cannot handle pixel mouse scrolls correctly!")
        }
        if ev.y < 0. {
            proj.scale *= 2.
        } else if ev.y > 0. {
            proj.scale /= 2.
        }
    }

    if mouse_button.pressed(MouseButton::Left) {
        let multiplier = 0.2 * proj.scale;
        for ev in mouse_motion.read() {
            transform.translation.x -= ev.delta.x * multiplier;
            transform.translation.y += ev.delta.y * multiplier;
        }
    }
}
