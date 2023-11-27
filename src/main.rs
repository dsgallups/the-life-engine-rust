#![allow(dead_code)]
mod life_engine;

use life_engine::LEWorld;
use startup::StartupPlugin;
mod startup;

use bevy::prelude::*;

fn main() {
    //println!("ozymandias");
    let world = LEWorld::default();

    App::new()
        .insert_resource(world)
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .add_plugins((DefaultPlugins, StartupPlugin))
        .add_systems(Update, (draw_cursor, frame_update))
        .add_systems(FixedUpdate, fixed_update)
        .run();
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 1., Color::WHITE);
}

fn frame_update(mut last_time: Local<f32>, time: Res<Time>) {
    // Default `Time` is `Time<Virtual>` here
    info!(
        "time since last frame_update: {}",
        time.elapsed_seconds() - *last_time
    );
    *last_time = time.elapsed_seconds();
}

fn fixed_update(mut last_time: Local<f32>, time: Res<Time>, fixed_time: Res<Time<Fixed>>) {
    // Default `Time`is `Time<Fixed>` here
    info!(
        "time since last fixed_update: {}\n",
        time.elapsed_seconds() - *last_time
    );

    info!("fixed timestep: {}\n", time.delta_seconds());
    // If we want to see the overstep, we need to access `Time<Fixed>` specifically
    info!(
        "time accrued toward next fixed_update: {}\n",
        fixed_time.overstep().as_secs_f32()
    );
    *last_time = time.elapsed_seconds();
}
