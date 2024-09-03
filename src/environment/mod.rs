pub use bevy::prelude::*;
use mouse_hover::hover_over_organism;

use crate::cell::CellType;

use super::{
    game::GameState,
    organism::{Organism, OrganismPlugin},
};

pub mod direction;
pub use direction::*;

pub mod mouse_hover;

#[allow(dead_code)]
#[derive(Resource, Debug)]
pub struct EnvironmentSettings {
    pub producer_threshold: u16,
    pub hunger_tick: u64,
    pub spawn_radius: u16,
    pub age_rate: u64,
    pub max_organisms: Option<usize>,
}

impl Default for EnvironmentSettings {
    fn default() -> Self {
        EnvironmentSettings {
            producer_threshold: 35,
            hunger_tick: 4300,
            spawn_radius: 15,
            age_rate: 8200,
            //max_organisms: Some(2000),
            max_organisms: None,
        }
    }
}

/// The main plugin for the game. This sets up several resources and systems
///
///
/// These are
/// - [`EnvironmentSettings`]: Provides some configuration about the behavior of organisms
/// - [`OccupiedLocations`]: a quick access hashmap for cells of organisms to interact with the environment.
///     This also allows organisms to know where they can spawn their genetic children and where they can move
///     (if they can move).
/// - [`Ticker`]: Determines when the organisms can "make a move". This is independent of framerate and ideally
///     should be configurable in [`EnvironmentSettings`].
/// - [`OrganismPlugin`] creates the update systems for all organisms in the environment.
///
/// The game starts with the first organism as laid out in [`spawn_first_organism`].
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnvironmentSettings::default())
            //.insert_resource(Ticker::new(Duration::from_millis(80)))
            .add_systems(
                OnEnter(GameState::Playing),
                (clear_background, spawn_first_organism),
            )
            //.add_systems(PreUpdate, tick_ticker)
            .add_systems(
                Update,
                hover_over_organism.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                escape_hatch.run_if(in_state(GameState::Playing)),
            )
            .add_plugins(OrganismPlugin);
    }
}

fn escape_hatch(mut commands: Commands, childless: Query<(Entity, &CellType), Without<Parent>>) {
    for (entity, cell_type) in &childless {
        if cell_type != &CellType::food() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/*
#[derive(Resource)]
pub struct Ticker(Timer, u64);

impl Ticker {
    pub fn new(rate: Duration) -> Self {
        Self(Timer::new(rate, TimerMode::Repeating), 0)
    }

    pub fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
    }

    pub fn just_finished(&self) -> bool {
        self.0.just_finished()
    }

    pub fn increment_tick(&mut self) {
        self.1 += 1;
    }

    pub fn current_tick(&self) -> u64 {
        self.1
    }
}

fn tick_ticker(mut ticker: ResMut<Ticker>, time: Res<Time>) {
    ticker.tick(time.delta());

    if ticker.just_finished() {
        ticker.increment_tick();
    }
}
*/
fn clear_background(mut color: ResMut<ClearColor>) {
    color.0 = Color::BLACK
}

/// Creates the first organism. [`Organism::insert_at`] is used to unify spawning of the organism in the ECS
/// as well as placing itself in the [`OccupiedLocations`] hashmap.
fn spawn_first_organism(mut commands: Commands) {
    Organism::first_organism().insert_at(&mut commands, Vec2::new(10., 10.));
}
