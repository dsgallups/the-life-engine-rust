use bevy::prelude::*;

use crate::{
    environment::{EnvironmentSettings, Ticker},
    game::GameState,
    neighbor::VecExt,
    CellTree,
};

use super::FoodBundle;

#[derive(Component, Default)]
pub struct ProducerCell {
    counter: u8,
}

pub struct ProducerPlugin;

impl Plugin for ProducerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, produce_food.run_if(in_state(GameState::Playing)));
    }
}

pub fn produce_food(
    mut commands: Commands,
    timer: Res<Ticker>,
    locations: Res<CellTree>,
    settings: Res<EnvironmentSettings>,
    mut producers: Query<(&mut ProducerCell, &GlobalTransform)>,
) {
    if !timer.just_finished() {
        return;
    }
    for (mut producer, global_location) in &mut producers {
        producer.counter += 1;
        let translation = global_location.translation();
        if producer.counter >= settings.producer_threshold {
            if let Some(free_space) = translation.get_free_space(&locations) {
                commands.spawn(FoodBundle::at(free_space));
            }

            producer.counter = 0;
        }
    }
}
