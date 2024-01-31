use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::components::{FpsText, MousePosBox};

pub fn move_camera(
    mut camera_query: Query<(&Camera, &GlobalTransform, &mut Transform)>,
    mouse_button: Res<Input<MouseButton>>,
    mut cursor_moved: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut mouse_pos_box: Query<&mut Text, With<MousePosBox>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform, mut transform) = camera_query.single_mut();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let mouse_down = mouse_button.pressed(MouseButton::Left);

    let mut pan = Vec2::ZERO;
    for ev in cursor_moved.read() {
        if mouse_down {
            pan += ev.delta;
        }
    }

    if mouse_down {
        transform.translation.x += -pan.x * 0.05;
        transform.translation.y += pan.y * 0.05;
    }

    let mut scroll = 0.0;

    for event in mouse_wheel.read() {
        scroll += event.y;
    }

    transform.scale.x += scroll * 0.01;
    transform.scale.y += scroll * 0.01;

    gizmos.circle_2d(point, 1., Color::WHITE);

    //print the mousie position
    let mut text = mouse_pos_box.get_single_mut().unwrap();

    text.sections[0].value = format!("({}, {})", point.x as i64, point.y as i64);
}

pub fn text_fps_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

pub fn frame_update(mut last_time: Local<f32>, time: Res<Time>) {
    // Default `Time` is `Time<Virtual>` here
    /*info!(
        "time since last frame_update: {}",
        time.elapsed_seconds() - *last_time
    );*/
    *last_time = time.elapsed_seconds();
}
