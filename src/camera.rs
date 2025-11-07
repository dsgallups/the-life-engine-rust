use bevy::{
    input::mouse::{AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
};

use crate::settings::Keybinds;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);

    app.add_systems(Update, (update_zoom, move_camera));
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

fn move_camera(
    input: Res<ButtonInput<KeyCode>>,
    keybinds: Res<Keybinds>,
    mut camera: Single<&mut Transform, With<WorldCamera>>,
    time: Res<Time>,
) {
    let mut del = Vec2::ZERO;
    if input.pressed(keybinds.key_up) {
        del.y = 1.;
    }
    if input.pressed(keybinds.key_down) {
        del.y -= 1.;
    }
    if input.pressed(keybinds.key_left) {
        del.x = -1.;
    }
    if input.pressed(keybinds.key_right) {
        del.x += 1.;
    }
    if del == Vec2::ZERO {
        return;
    }

    let del = del * camera.scale.xy() * time.delta_secs() * 200.;

    camera.translation.x += del.x;
    camera.translation.y += del.y;
}
