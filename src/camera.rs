use bevy::{
    input::mouse::{AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);

    app.add_systems(Update, update_zoom);
}

#[derive(Component)]
pub struct WorldCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        WorldCamera,
        Camera2d,
        Transform::from_scale(Vec3::splat(0.05)),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
    ));
}

fn update_zoom(
    input: Res<AccumulatedMouseScroll>,
    camera: Query<&mut Transform, With<WorldCamera>>,
) {
    let acc = match input.unit {
        MouseScrollUnit::Line => input.delta,
        MouseScrollUnit::Pixel => input.delta,
    };
    if acc == Vec2::ZERO {
        return;
    }
    for mut transform in camera {
        if acc.y < 0. {
            if transform.scale.y == -0.05 {
                transform.scale += Vec3::splat(0.1);
            } else {
                transform.scale += Vec3::splat(0.05);
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if transform.scale.y == 0.05 {
                transform.scale -= Vec3::splat(0.1);
            } else {
                transform.scale -= Vec3::splat(0.05);
            }
        }
    }
}
