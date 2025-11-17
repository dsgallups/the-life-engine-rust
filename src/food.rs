use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionStart, Sensor};
use bevy::{color::palettes::tailwind::BLUE_600, prelude::*};
use rand::Rng;

use crate::{
    cell::Cells,
    organism::{Dead, Organism, SpawnOrganism},
    utils::Random,
};

#[derive(Component, Reflect, Default)]
pub struct FoodEaten(pub u32);

#[derive(Component, Reflect)]
pub struct Health(pub u32);

impl Default for Health {
    fn default() -> Self {
        Self(50 * 60)
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<FoodAssets>()
        .init_resource::<FoodTimer>();
    app.add_systems(Update, spawn_food)
        .add_systems(PostUpdate, (on_full_belly, update_health));
}

#[derive(Resource)]
struct FoodAssets {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}
impl FromWorld for FoodAssets {
    fn from_world(world: &mut World) -> Self {
        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Rectangle::from_length(1.));
        let material = world
            .resource_mut::<Assets<ColorMaterial>>()
            .add(Color::from(BLUE_600));
        Self { mesh, material }
    }
}

#[derive(Resource, Default)]
pub struct FoodTimer {
    timer: u32,
}

#[derive(Component)]
pub struct Food;

fn spawn_food(
    mut commands: Commands,
    assets: Res<FoodAssets>,
    mut food_timer: ResMut<FoodTimer>,
    mut rand: ResMut<Random>,
) {
    food_timer.timer += 1;
    if food_timer.timer < 1000 {
        return;
    }
    food_timer.timer = 0;
    let x = rand.0.random_range(-90_f32..=90_f32);
    let y = rand.0.random_range(-90_f32..=90_f32);

    commands
        .spawn((
            Food,
            Sensor,
            CollisionEventsEnabled,
            Collider::rectangle(1., 1.),
            Mesh2d(assets.mesh.clone()),
            MeshMaterial2d(assets.material.clone()),
            Transform::from_xyz(x, y, 0.),
        ))
        .observe(food_collision);
}

fn food_collision(
    ev: On<CollisionStart>,
    mut commands: Commands,
    mut organisms: Query<(&mut Health, &mut FoodEaten)>,
) {
    if let Some(body) = ev.body2
        && let Ok((mut health, mut food_eaten)) = organisms.get_mut(body)
    {
        health.0 += 25 * 60;
        food_eaten.0 += 1;
        commands.entity(ev.collider1).try_despawn();
    };
}

fn on_full_belly(
    organisms: Query<(&Organism, &mut FoodEaten, &Cells), Without<Dead>>,
    mut spawn_cmd: MessageWriter<SpawnOrganism>,
    mut rand: ResMut<Random>,
) {
    for (organism, mut food_eaten, cells) in organisms {
        let num_cells = cells.cells().len() as u32;
        if food_eaten.0 >= num_cells {
            food_eaten.0 -= num_cells;

            let x = rand.0.random_range(-75_f32..=75_f32);
            let y = rand.0.random_range(-75_f32..=75_f32);
            let mut genome = organism.genome().deep_clone();
            genome.scramble(&mut rand.0);

            spawn_cmd.write(SpawnOrganism::new(genome, Vec2::new(x, y)));
        }
    }
}

fn update_health(health: Query<&mut Health>) {
    for mut health in health {
        health.0 = health.0.saturating_sub(1);
    }
}
