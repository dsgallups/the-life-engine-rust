use bevy::prelude::*;
use bevy_spatial::SpatialAccess;

use crate::{
    environment::{EnvironmentSettings, Ticker},
    game::GameState,
    neighbor::{KDTreeExt as _, VecExt},
    organism::{genome::CellLocation, Organism},
    CellTree,
};

use super::{CellType, EnvironmentCellType, FoodBundle};

#[derive(Component, Default)]
pub struct ProducerCell {
    counter: u8,
}

pub struct ProducerPlugin;

impl Plugin for ProducerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, produce_food.run_if(in_state(GameState::Playing)));
        //app.add_systems(Update, test_nntree.run_if(in_state(GameState::Playing)));
    }
}

/*pub fn test_nntree(tree: Res<KDTree2<CellType>>) {
    let nns = tree.k_nearest_neighbour(Vec2::new(0., 0.), 3);
    info!("nearest neighbors:");
    for n in nns {
        info!("({}, {:?})", n.0, n.1);
    }
}*/

pub fn produce_food(
    mut commands: Commands,
    timer: Res<Ticker>,
    locations: Res<CellTree>,
    settings: Res<EnvironmentSettings>,
    mut producers: Query<(&mut ProducerCell, &GlobalTransform, &Parent)>,
    organisms: Query<&Transform, With<Organism>>,
) {
    if !timer.just_finished() {
        return;
    }
    for (mut producer, global_location, producer_parent) in &mut producers {
        let Ok(organism_location) = organisms.get(producer_parent.get()) else {
            panic!("Orphan cell found!");
        };
        producer.counter += 1;
        let translation = global_location.translation();
        info!("for producer at {:?}", translation);

        for (i, (location, neighbor)) in locations.closest_neighbors(translation).enumerate() {
            info!("Neighbor {}: ({}, {:?})", i, location, neighbor);
        }

        info!("\n\nEND\n\n");

        /*for food_location in Neighbors::around(translation.as_vec2()) {
            if locations.k_nearest_neighbour(, k)
        }*/

        /*if producer.counter >= settings.producer_threshold {
            for location in global_location_of_producer.around() {
                if !occupied_locations.occupied(&location) {
                    //make food at an unoccupied location
                    let new_food = commands.spawn(FoodBundle::at(location)).id();
                    occupied_locations.insert(location, new_food, EnvironmentCellType::Food);
                    break;
                }
            }
            producer.counter = 0;
        }*/
    }
}
