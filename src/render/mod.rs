use crate::LEWorld;
use bevy::prelude::*;
use bevy::{
    app::{App, FixedUpdate, Update},
    ecs::{
        event::EventReader,
        system::{Commands, Local, Query, Res, ResMut},
    },
    gizmos::gizmos::Gizmos,
    input::{
        mouse::{MouseButton, MouseMotion, MouseWheel},
        Input,
    },
    render::{camera::Camera, color::Color},
    time::{Fixed, Time},
    transform::components::{GlobalTransform, Transform},
    window::Window,
    DefaultPlugins,
};
mod startup;
use startup::StartupPlugin;

pub fn begin_ticking(world: LEWorld) {
    App::new()
        .insert_resource(world)
        .insert_resource(Time::<Fixed>::from_seconds(0.2))
        .add_plugins((DefaultPlugins, StartupPlugin))
        .add_systems(Update, (move_camera, frame_update))
        .add_systems(FixedUpdate, fixed_update)
        .run();
}

fn move_camera(
    mut commands: Commands,
    mut camera_query: Query<(&Camera, &GlobalTransform, &mut Transform)>,
    mouse_button: Res<Input<MouseButton>>,
    mut cursor_moved: EventReader<MouseMotion>,
    mut sprites_query: Query<(Entity, &Sprite)>,
    mut mouse_wheel: EventReader<MouseWheel>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
    mut world: ResMut<LEWorld>,
) {
    if let Err(e) = world.tick() {
        panic!("{}", e);
    }

    let new_sprites = world.draw();

    for (ent, _sprites) in &mut sprites_query {
        commands.entity(ent).despawn();
    }

    commands.spawn_batch(new_sprites);
    let (camera, camera_transform, mut transform) = camera_query.single_mut();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let mouse_down = mouse_button.pressed(MouseButton::Left);

    if mouse_down {
        let mut pan = Vec2::ZERO;
        for ev in cursor_moved.read() {
            pan += ev.delta;
        }
        if pan.x.abs() <= 30. && pan.y.abs() <= 30. {
            transform.translation.x += -pan.x * 0.05;
            transform.translation.y += pan.y * 0.05;
        }
    }

    let mut scroll = 0.0;

    for event in mouse_wheel.read() {
        scroll += event.y;
    }

    transform.scale.x += scroll * 0.01;
    transform.scale.y += scroll * 0.01;

    gizmos.circle_2d(point, 1., Color::WHITE);
}

fn frame_update(mut last_time: Local<f32>, time: Res<Time>) {
    // Default `Time` is `Time<Virtual>` here
    /*info!(
        "time since last frame_update: {}",
        time.elapsed_seconds() - *last_time
    );*/
    *last_time = time.elapsed_seconds();
}

fn fixed_update(
    mut commands: Commands,
    mut last_time: Local<f32>,
    mut sprites_query: Query<(Entity, &Sprite)>,
    time: Res<Time>,
    _fixed_time: Res<Time<Fixed>>,
    mut world: ResMut<LEWorld>,
) {
    //let _ = world.tick();
    //world.draw(&mut commands);
    // Default `Time`is `Time<Fixed>` here
    /*info!(
        "time since last fixed_update: {}\n",
        time.elapsed_seconds() - *last_time
    );

    info!("fixed timestep: {}\n", time.delta_seconds());
    // If we want to see the overstep, we need to access `Time<Fixed>` specifically
    info!(
        "time accrued toward next fixed_update: {}\n",
        fixed_time.overstep().as_secs_f32()
    );*/
    *last_time = time.elapsed_seconds();
}
