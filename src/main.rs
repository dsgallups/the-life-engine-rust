pub mod cell;
//pub mod messages;
mod map;
pub mod organism;
mod startup;
use map::WorldLocation;
use neighbors::NEIGHBORS;
use rand::{thread_rng, Rng};
use startup::{RemoveFood, StartupPlugin};
mod direction;
mod neighbors;

mod world_settings;

use bevy::{
    app::AppExit,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
    utils::{HashMap, HashSet},
};

pub use cell::*;
//pub use messages::*;
pub use organism::*;
use world_settings::WorldSettings;

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_seconds(0.05))
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin, StartupPlugin))
        .add_systems(
            Update,
            (
                move_camera,
                frame_update,
                text_fps_system,
                remove_food_system,
                reproduce_system,
            ),
        )
        .add_systems(FixedUpdate, (fixed_update, organ_system))
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
/*
fn tick_organisms(
    mut ev_organism: EventWriter<OrganismEvent>,
    world_settings: Res<WorldSettings>,
    mut organism_query: Query<
        (
            Entity,
            &WorldLocation,
            &mut OrganismInfo,
            &OrganismType,
            &Children,
        ),
        With<OrganismType>,
    >,
    mut organ_query: Query<(&WorldLocation, &mut OrganType)>,
) {
    for (org_ent, location, mut organism_info, organism_type, children) in organism_query.iter_mut()
    {
        organism_info.time_alive += 1;
        organism_info.time_since_consumption += 1;

        if organism_info.belly == 0 {
            println!("im going to starve: {:?}", org_ent);
            ev_organism.send(OrganismEvent(org_ent, OrganismRequest::Starve));
        } else if organism_info.time_since_consumption % world_settings.hunger_tick == 0 {
            organism_info.belly -= 1;
        }

        if organism_info.belly >= children.len() as u64 {
            ev_organism.send(OrganismEvent(org_ent, OrganismRequest::Reproduce));
        }

        let mut eye_locations: Option<Vec<(WorldLocation, direction::Direction)>> = None;

        for child in children.iter() {
            let (relative_location, mut organ) = organ_query.get_mut(*child).unwrap();

            match organ.as_mut() {
                OrganType::Producer(ref mut producer) => {
                    if *organism_type == OrganismType::Mover {
                        continue;
                    }
                    if thread_rng().gen_range(0..=100) <= world_settings.producer_probability {
                        ev_organism.send(OrganismEvent(
                            org_ent,
                            OrganismRequest::ProduceFoodAround(*location + *relative_location),
                        ));
                        producer.counter = 0;
                    }
                }
                OrganType::Eye(direction) => {
                    if eye_locations.is_none() {
                        eye_locations = Some(Vec::new());
                    }
                    eye_locations
                        .as_mut()
                        .unwrap()
                        .push((*location + *relative_location, *direction));
                }
                OrganType::Killer => {
                    ev_organism.send(OrganismEvent(
                        org_ent,
                        OrganismRequest::KillAround(*location + *relative_location),
                    ));
                }
                OrganType::Mouth => {
                    ev_organism.send(OrganismEvent(
                        org_ent,
                        OrganismRequest::EatFoodAround(*location + *relative_location),
                    ));
                }

                _ => {}
            }
        }
        if *organism_type == OrganismType::Mover {
            if let Some(eye_locations) = eye_locations {
                ev_organism.send(OrganismEvent(
                    org_ent,
                    OrganismRequest::IntelligentMove(eye_locations),
                ));
            } else {
                ev_organism.send(OrganismEvent(
                    org_ent,
                    OrganismRequest::MoveBy(WorldLocation::new(
                        rand::random::<i64>() % 2,
                        rand::random::<i64>() % 2,
                    )),
                ));
            }
        }
    }
}
*/
fn organ_system(
    mut commands: Commands,
    world_settings: Res<WorldSettings>,
    food_query: Query<(Entity, &WorldLocation), With<Food>>,
    map_query: Query<(Entity, &WorldLocation)>,
    organ_query: Query<(&WorldLocation, &OrganType, &Parent)>,
    mut organism_event: EventWriter<Reproduce>,
    mut remove_food: EventWriter<RemoveFood>,
    mut organism_query: Query<(Entity, &WorldLocation, &mut OrganismInfo, &Children)>,
) {
    let mut food: HashMap<WorldLocation, Entity> = HashMap::new();
    let mut map: HashMap<WorldLocation, Entity> = HashMap::new();

    map_query.iter().for_each(|(ent, location)| {
        map.insert(*location, ent);
    });

    for (ent, location) in food_query.iter() {
        food.insert(*location, ent);
    }

    for (rel_location, organ_type, parent) in organ_query.iter() {
        let (org_ent, organism_loc, mut organism_info, children) =
            organism_query.get_mut(**parent).unwrap();

        if organism_info.belly == 0 {
            commands.entity(org_ent).despawn_recursive();
            continue;
        }

        let absolute_location = *organism_loc + *rel_location;

        match organ_type {
            OrganType::Mouth => {
                for around in NEIGHBORS.adjacent {
                    let checking_loc = absolute_location + around;
                    if let Some(food_ent) = food.get(&checking_loc) {
                        organism_info.belly += 1;

                        remove_food.send(RemoveFood(*food_ent));

                        if organism_info.belly > children.len() as u64 {
                            organism_event.send(Reproduce(**parent));
                        }
                    }
                }
            }
            OrganType::Producer => {
                if thread_rng().gen_range(0..=100) <= world_settings.producer_probability {
                    for around in NEIGHBORS.adjacent {
                        let checking_loc = absolute_location + around;
                        if map.get(&checking_loc).is_none() {
                            commands.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: Food.color(),
                                        ..default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        checking_loc.x() as f32,
                                        checking_loc.y() as f32,
                                        0.,
                                    )),
                                    ..default()
                                },
                                checking_loc,
                                Food,
                            ));
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn reproduce_system(
    mut commands: Commands,
    world_settings: Res<WorldSettings>,
    mut requests: EventReader<Reproduce>,
    map_query: Query<&WorldLocation>,
    mut organism_query: Query<(&WorldLocation, &mut OrganismInfo, &Children)>,
    organ_query: Query<(&WorldLocation, &OrganType)>,
) {
    let mut map: HashSet<WorldLocation> = HashSet::new();

    let mut rng = rand::thread_rng();

    map_query.iter().for_each(|location| {
        map.insert(*location);
    });

    'request: for request in requests.read() {
        let (organism_location, mut organism_info, children) =
            organism_query.get_mut(request.0).unwrap();

        let new_organism_stats = organism_info.gen_child_stats();
        let mutation_actions = MutationAction::rand_list(new_organism_stats.mutation_rate);

        let mut new_organs = children
            .iter()
            .map(|child| {
                let (relative_location, organ) = organ_query.get(*child).unwrap();
                (*relative_location, organ.clone())
            })
            .collect::<Vec<_>>();

        let turn_amount = organism_info.facing.turn(new_organism_stats.facing);

        for organ in new_organs.iter_mut() {
            let loc = &mut organ.0;
            let og = *loc;
            match turn_amount {
                -1 => {
                    //counter-clockwise
                    loc.set_x(-og.y());
                    loc.set_y(og.x());
                }
                1 => {
                    //clockwise
                    loc.set_x(og.y());
                    loc.set_y(-og.x());
                }
                2 => {
                    //opposite
                    loc.set_x(-og.x());
                    loc.set_y(-og.y());
                }
                0 => {
                    //none
                }
                _ => unreachable!(),
            }
        }

        for mutation in mutation_actions {
            match mutation {
                MutationAction::Delete => {
                    if new_organs.is_empty() {
                        continue 'request;
                    }
                    let index = rng.gen_range(0..new_organs.len());
                    new_organs.swap_remove(index);
                }

                MutationAction::New => {
                    let occupied_locations = new_organs.iter().map(|o| o.0).collect::<Vec<_>>();

                    let attach_to = if occupied_locations.is_empty() {
                        WorldLocation::new(0, 0)
                    } else {
                        //pick a random location in the list
                        *occupied_locations
                            .get(rng.gen_range(0..occupied_locations.len()))
                            .unwrap()
                    };

                    //pick a random place to start
                    let mut x = rng.gen_range(-1..=1);
                    let mut y = rng.gen_range(-1..=1);
                    if x == 0 && y == 0 {
                        if rng.gen::<bool>() {
                            x = if rng.gen::<bool>() { 1 } else { -1 };
                        } else {
                            y = if rng.gen::<bool>() { 1 } else { -1 };
                        }
                    }

                    let mut count = 0;
                    loop {
                        if count > 11 {
                            continue 'request;
                        }
                        if occupied_locations.contains(&(attach_to + WorldLocation::new(x, y))) {
                            if x == 1 {
                                if y == -1 {
                                    y = 0
                                } else if y == 0 {
                                    y = 1
                                } else if y == 1 {
                                    x = 0
                                }
                            } else if x == 0 {
                                if y == -1 {
                                    x = 1
                                } else if y == 1 {
                                    x = -1
                                }
                            } else if x == -1 {
                                if y == -1 {
                                    x = 0;
                                } else if y == 0 {
                                    y = -1;
                                } else if y == 1 {
                                    y = 0;
                                }
                            }
                            count += 1;
                        } else {
                            new_organs.push((WorldLocation::new(x, y), OrganType::new_rand()));
                            break;
                        }
                    }
                }
                MutationAction::MutateOrgan => {
                    let new_organs_len = new_organs.len();
                    let organ_to_mutate = new_organs
                        .get_mut(rng.gen_range(0..new_organs_len))
                        .unwrap();
                    organ_to_mutate.1.mutate();
                }
            }
        }

        //now try to spawn the organism by finding an empty location for all of its organs
        let basis = *organism_location;
        let mut attempt_count = 0;
        let baby_location: WorldLocation = loop {
            let x = rng.gen_range(
                -(world_settings.spawn_radius as i64)..=world_settings.spawn_radius as i64,
            );
            let y = rng.gen_range(
                -(world_settings.spawn_radius as i64)..=world_settings.spawn_radius as i64,
            );
            let new_basis = basis + WorldLocation::new(x, y);
            //there could be an instance where it's on the edge and this doesn't work, so then the insert organism code should prevent the other case.
            if let Some(wall_half) = world_settings.wall_length_half {
                if new_basis.x() <= -wall_half
                    || new_basis.x() >= wall_half
                    || new_basis.y() <= -wall_half
                    || new_basis.y() >= wall_half
                {
                    continue;
                }
            }

            let mut valid_basis = true;

            for (rel, _organ) in new_organs.iter() {
                match map.get(&(new_basis + *rel)) {
                    None => {}
                    _ => {
                        valid_basis = false;
                    }
                }
            }
            if valid_basis {
                break new_basis;
            }
            attempt_count += 1;
            if attempt_count == 10 {
                continue 'request;
            }
        };

        let new_organism_type = if new_organs.iter().any(|(_, o)| o == &OrganType::Mover) {
            OrganismType::Mover
        } else {
            OrganismType::Producer
        };

        println!(
            "spawned new organism at {:?} with {} organs",
            baby_location,
            new_organs.len()
        );

        commands
            .spawn(OrganismBundle::new(
                new_organism_type,
                baby_location,
                new_organism_stats,
            ))
            .with_children(|parent| {
                for (rel, organ) in new_organs {
                    println!("rel: {:?}, organ: {:?}", rel, organ);
                    parent.spawn(OrganBundle::new(organ, rel));
                }
            });
    }
}

fn remove_food_system(mut commands: Commands, mut remove_food: EventReader<RemoveFood>) {
    for remove in remove_food.read() {
        commands.entity(remove.0).despawn();
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
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
