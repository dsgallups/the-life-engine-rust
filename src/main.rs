pub mod cell;
//pub mod messages;
mod map;
pub mod organism;
mod startup;
use map::WorldMap;
use startup::StartupPlugin;
mod direction;
mod neighbors;

mod event;
mod world_settings;

use bevy::{
    app::AppExit,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
        ButtonState,
    },
    math::I64Vec2,
    prelude::*,
};

pub use cell::*;
//pub use messages::*;
pub use event::*;
pub use organism::*;
use world_settings::WorldSettings;

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_seconds(0.05))
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin, StartupPlugin))
        .add_systems(Update, (move_camera, frame_update, text_fps_system))
        .add_systems(FixedUpdate, (tick_organisms, fixed_update))
        .run();
}
fn move_camera(
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

fn text_fps_system(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn frame_update(mut last_time: Local<f32>, time: Res<Time>) {
    // Default `Time` is `Time<Virtual>` here
    /*info!(
        "time since last frame_update: {}",
        time.elapsed_seconds() - *last_time
    );*/
    *last_time = time.elapsed_seconds();
}

fn tick_organisms(
    mut commands: Commands,
    mut ev_organism: EventWriter<OrganismEvent>,
    world_settings: Res<WorldSettings>,
    mut organism_query: Query<(Entity, &mut Organism)>,
) {
    for (ent, mut organism) in organism_query.iter_mut() {
        let requests = organism.tick(&world_settings);

        if requests.contains(&OrganismRequest::Starve) {
            commands.entity(ent).despawn_recursive();
        } else {
            for request in requests {
                ev_organism.send(OrganismEvent(ent, request));
            }
        }
    }
}

fn process_requests(
    mut commands: Commands,
    mut requests: EventReader<OrganismEvent>,
    mut map_query: Query<&mut Transform, With<Cell>>,
    mut organism_query: Query<(Entity, &mut Organism)>,
) {
    for request in requests.read() {
        let organism = organism_query.get_mut(request.0).unwrap();

        match request.1 {
            OrganismRequest::ProduceFoodAround(location) => {
                //the organism will have depleted the food production for whichever of its cells produced
                if let Some((food_location, cell)) = map.insert_food_around(location) {
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: cell.color(),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                food_location.x as f32,
                                food_location.y as f32,
                                0.,
                            )),
                            ..default()
                        },
                        cell,
                    ));
                }
                //check map
            }
            _ => todo!(),
        }
    }
}

fn fixed_update(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut last_time: Local<f32>,
    mut mouse_button: EventReader<MouseButtonInput>,
    mut key_presses: EventReader<KeyboardInput>,
    mut sprites_query: Query<(Entity, &Sprite)>,
    mut _exit: EventWriter<AppExit>,
    windows: Query<&Window>,
    time: Res<Time>,
    _fixed_time: Res<Time<Fixed>>,
    //mut world: ResMut<LEWorld>,
) {
    /*
    if let Some(event) = mouse_button.read().next() {
        if event.button == MouseButton::Right && event.state == ButtonState::Pressed {
            if !world.paused {
                world.pause()
            } else {
                world.log()
            }
        } else if event.button == MouseButton::Middle && event.state == ButtonState::Pressed {
            world.unpause();
        } else if event.button == MouseButton::Left
            && event.state == ButtonState::Pressed
            && world.paused
        {
            let (camera, camera_transform) = camera_query.single();
            //get the location of the cursor
            let Some(cursor_position) = windows.single().cursor_position() else {
                return;
            };

            let Some(cursor_position) =
                camera.viewport_to_world_2d(camera_transform, cursor_position)
            else {
                return;
            };
            let position = I64Vec2::new(cursor_position.x as i64, cursor_position.y as i64);
            println!("Cursor position: {}", position);

            world.log_square(position);
        }
    }
    if let Some(event) = key_presses.read().next() {
        if let Some(key_code) = event.key_code {
            if world.paused && event.state == ButtonState::Pressed {
                match key_code {
                    KeyCode::R => world.reset(),
                    KeyCode::D => world.decimate(),
                    KeyCode::L => world.limit_organism_population(Some(200)),
                    KeyCode::B => world.limit_organism_population(None),
                    KeyCode::C => world.check_alive(),
                    KeyCode::M => world.postmortem(),
                    _ => {}
                }
            }
        }
    }

    if let Err(e) = world.tick() {
        println!("Error ticking world:\n{}", e);
        world.pause();
    }

    let new_sprites = world.draw();

    for (ent, _sprites) in &mut sprites_query {
        commands.entity(ent).despawn();
    }

    commands.spawn_batch(new_sprites);
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
    */
    *last_time = time.elapsed_seconds();
}

#[derive(Component)]
pub struct MousePosBox;

#[derive(Component)]
pub struct FpsText;
