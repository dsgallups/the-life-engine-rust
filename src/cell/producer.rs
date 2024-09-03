use bevy::prelude::*;

use crate::{environment::EnvironmentSettings, neighbor::VecExt, CellTree, GameState};

use super::FoodBundle;

#[derive(Component, Default)]
pub struct ProducerCell {
    counter: u16,
}

pub struct ProducerPlugin;

impl Plugin for ProducerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, produce_food.run_if(in_state(GameState::Playing)));
    }
}

pub fn produce_food(
    mut commands: Commands,
    locations: Res<CellTree>,
    settings: Res<EnvironmentSettings>,
    mut producers: Query<(&mut ProducerCell, &GlobalTransform)>,
) {
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
