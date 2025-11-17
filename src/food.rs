use std::time::Duration;

use bevy::{color::palettes::tailwind::BLUE_600, prelude::*};
use rand::Rng;

use crate::utils::Random;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<FoodAssets>()
        .init_resource::<FoodTimer>();
    app.add_systems(Update, spawn_food);
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

#[derive(Resource)]
pub struct FoodTimer {
    timer: Timer,
}
impl Default for FoodTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Food;

fn spawn_food(
    mut commands: Commands,
    assets: Res<FoodAssets>,
    mut food_timer: ResMut<FoodTimer>,
    mut rand: ResMut<Random>,
    time: Res<Time>,
) {
    food_timer.timer.tick(time.delta());
    if !food_timer.timer.just_finished() {
        return;
    }
    let x = rand.0.random_range(-50_f32..=50_f32);
    let y = rand.0.random_range(-50_f32..=50_f32);

    commands.spawn((
        Food,
        Mesh2d(assets.mesh.clone()),
        MeshMaterial2d(assets.material.clone()),
        Transform::from_xyz(x, y, 0.),
    ));
}
