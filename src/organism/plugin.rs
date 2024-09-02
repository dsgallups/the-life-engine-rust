use bevy::prelude::*;

use crate::{
    cell::{CellType, EnvironmentCellType, FoodBundle, MouthPlugin, ProducerPlugin},
    environment::{EnvironmentSettings, Ticker},
    game::GameState,
};

use super::{genome::CellLocation, Organism};

/// Combines the systems of cells and organism actions
pub struct OrganismPlugin;

impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut App) {
        /*app.add_plugins((ProducerPlugin, MouthPlugin, MoverPlugin))
        .add_systems(
                    Update,
                    (
                        starve_organism.run_if(in_state(GameState::Playing)),
                        reproduce_organism.run_if(in_state(GameState::Playing)),
                    ),
            );*/
        app.add_plugins((ProducerPlugin, MouthPlugin));
    }
}
/*
fn starve_organism(
    mut commands: Commands,
    mut occupied_locations: ResMut<OccupiedLocations>,
    settings: Res<EnvironmentSettings>,
    timer: Res<Ticker>,
    mut organisms: Query<(Entity, &mut Organism, &GlobalCellLocation)>,
    cells: Query<(&Parent, &CellLocation)>,
) {
    if !timer.just_finished() {
        return;
    }

    for (organism_entity, mut organism, organism_location) in &mut organisms {
        let ticks_alive = timer.current_tick() - organism.tick_born();
        if ticks_alive % settings.hunger_tick == 0 {
            // based on the age of the organism, we out how much food it should lose
            let age_cost = (ticks_alive / settings.age_rate) + 1;
            organism.lost_food(age_cost);
        }

        if organism.belly() == 0 {
            //before the organism dies, we need to turn the children
            //into food :D
            for (cell_parent, cell_location) in &cells {
                if organism_entity == cell_parent.get() {
                    //make food here
                    let global_cell_location = *organism_location + *cell_location;
                    let new_food = commands.spawn(FoodBundle::at(global_cell_location)).id();
                    occupied_locations.insert(
                        global_cell_location,
                        new_food,
                        EnvironmentCellType::Food,
                    );
                }
            }

            commands.entity(organism_entity).despawn_recursive();
        }
    }
}

fn reproduce_organism(
    mut commands: Commands,
    mut occupied_locations: ResMut<OccupiedLocations>,
    settings: Res<EnvironmentSettings>,
    timer: Res<Ticker>,
    mut organisms: Query<(&mut Organism, &GlobalCellLocation)>,
) {
    if !timer.just_finished() {
        return;
    }

    for (mut organism, organism_location) in &mut organisms {
        if organism.ready_to_reproduce() {
            let Some(new_organism) = organism.reproduce(timer.current_tick()) else {
                continue;
            };
            let mut new_organism_location = None;

            //info!("the organism has created a child!");
            //the organism gets three chances to place within the radius of the parent.
            //otherwise, it dies.
            'chance: for _ in 0..=2 {
                let chosen_location = organism_location.rand_around(settings.spawn_radius);

                for pot_child_locs in new_organism.occupying_locations() {
                    let global_child_loc = chosen_location + pot_child_locs;
                    //children can replace food
                    if occupied_locations
                        .cell_type_at(&global_child_loc)
                        .is_some_and(|cell_type| cell_type != CellType::food())
                    {
                        continue 'chance;
                    }
                }

                // this is a valid location

                new_organism_location = Some(chosen_location);
                break;
            }

            let Some(final_child_location) = new_organism_location else {
                //tough
                info!("The child did not survive birth");
                continue;
            };

            new_organism.insert_at(&mut commands, &mut occupied_locations, final_child_location);
            //info!("Child spawned!");
        }
    }
}
*/
