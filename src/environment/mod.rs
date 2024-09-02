use std::time::Duration;

pub use bevy::prelude::*;
use location::{GlobalCellLocation, OccupiedLocations};

use super::{
    game::GameState,
    organism::{Organism, OrganismPlugin},
};

pub mod direction;
pub use direction::*;
pub mod location;

#[allow(dead_code)]
#[derive(Resource, Debug)]
pub struct EnvironmentSettings {
    pub producer_threshold: u8,
    pub hunger_tick: u64,
    pub spawn_radius: u16,
    pub age_rate: u64,
    pub max_organisms: Option<usize>,
}

impl Default for EnvironmentSettings {
    fn default() -> Self {
        EnvironmentSettings {
            producer_threshold: 8,
            hunger_tick: 14,
            spawn_radius: 15,
            age_rate: 50,
            max_organisms: Some(10),
        }
    }
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnvironmentSettings::default())
            .insert_resource(OccupiedLocations::default())
            .insert_resource(Ticker::new(Duration::from_millis(80)))
            .add_systems(
                OnEnter(GameState::Playing),
                (clear_background, spawn_first_organism),
            )
            .add_systems(PreUpdate, tick_ticker)
            .add_plugins(OrganismPlugin);
    }
}

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

fn clear_background(mut color: ResMut<ClearColor>) {
    color.0 = Color::BLACK
}

fn spawn_first_organism(mut commands: Commands, mut global_positions: ResMut<OccupiedLocations>) {
    Organism::first_organism().insert_at(
        &mut commands,
        &mut global_positions,
        GlobalCellLocation::new(0, 0),
    );
}
