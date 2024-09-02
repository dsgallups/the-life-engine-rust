use bevy::prelude::*;

use crate::{
    environment::{EnvironmentSettings, Ticker},
    game::GameState,
    organism::{genome::CellLocation, Organism},
};

use super::{EnvironmentCellType, FoodBundle};

#[derive(Component, Default)]
pub struct ProducerCell {
    counter: u8,
}

pub struct ProducerPlugin;

impl Plugin for ProducerPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, produce_food.run_if(in_state(GameState::Playing)));
        todo!();
    }
}
/*
pub fn produce_food(
    mut commands: Commands,
    timer: Res<Ticker>,
    settings: Res<EnvironmentSettings>,
    mut producers: Query<(&mut ProducerCell, &CellLocation, &Parent)>,
    organisms: Query<&Transform, With<Organism>>,
) {
    if !timer.just_finished() {
        return;
    }
    for (mut producer, local_location, producer_parent) in &mut producers {
        let Ok(organism_location) = organisms.get(producer_parent.get()) else {
            panic!("Orphan cell found!");
        };
        producer.counter += 1;

        let global_location_of_producer = *organism_location + *local_location;

        if producer.counter >= settings.producer_threshold {
            for location in global_location_of_producer.around() {
                if !occupied_locations.occupied(&location) {
                    //make food at an unoccupied location
                    let new_food = commands.spawn(FoodBundle::at(location)).id();
                    occupied_locations.insert(location, new_food, EnvironmentCellType::Food);
                    break;
                }
            }
            producer.counter = 0;
        }
    }
}
*/
