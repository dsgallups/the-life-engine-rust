use bevy::prelude::*;
use bevy_spatial::SpatialAccess;

use crate::{
    cell::{Food, FoodBundle, KillerPlugin, MouthPlugin, MoverPlugin, ProducerPlugin},
    environment::{EnvironmentSettings, Ticker},
    game::GameState,
    neighbor::VecExt as _,
    CellTree,
};

use super::Organism;

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
        app.add_plugins((ProducerPlugin, MouthPlugin, MoverPlugin, KillerPlugin))
            .add_systems(
                Update,
                (
                    starve_organism.run_if(in_state(GameState::Playing)),
                    reproduce_organism.run_if(in_state(GameState::Playing)),
                ),
            );
    }
}

fn starve_organism(
    mut commands: Commands,
    settings: Res<EnvironmentSettings>,
    timer: Res<Ticker>,
    mut organisms: Query<(Entity, &Children, &mut Organism)>,
    locations: Query<&GlobalTransform>,
) {
    if !timer.just_finished() {
        return;
    }

    for (organism_entity, children, mut organism) in &mut organisms {
        let ticks_alive = timer.current_tick() - organism.tick_born();
        if ticks_alive % settings.hunger_tick == 0 {
            // based on the age of the organism, we out how much food it should lose
            let age_cost = (ticks_alive / settings.age_rate) + 1;
            organism.lost_food(age_cost);
        }

        if organism.belly() == 0 {
            //before the organism dies, we need to turn the children
            //into food :D
            // because it could be killed in the meantime or whatnot, we should just lay down the food here
            for child in children.iter() {
                let location = locations.get(*child).unwrap();
                commands.spawn(FoodBundle::at(location.translation()));
            }

            commands.entity(organism_entity).despawn_recursive();
        }
    }
}

fn reproduce_organism(
    mut commands: Commands,
    settings: Res<EnvironmentSettings>,
    timer: Res<Ticker>,
    tree: Res<CellTree>,
    mut organisms: Query<(&GlobalTransform, &mut Organism)>,
    food: Query<&Food>,
) {
    if !timer.just_finished()
        || settings
            .max_organisms
            .is_some_and(|max| organisms.iter().count() >= max)
    {
        return;
    }

    for (organism_transform, mut organism) in &mut organisms {
        if organism.ready_to_reproduce() {
            let Some(new_organism) = organism.reproduce(timer.current_tick()) else {
                continue;
            };

            let organism_location = organism_transform.translation();

            //info!("the organism has created a child!");
            //the organism gets three chances to place within the radius of the parent.
            //otherwise, it dies.
            let mut chance = 0;
            let new_organism_location = 'find: loop {
                if chance > 2 {
                    break None;
                }
                let random_location = organism_location.rand_around(settings.spawn_radius);

                for (_, e) in tree.within_distance(random_location, organism.radius() as f32) {
                    // children can spawn over food
                    // this will clean up food anyway
                    if let Some(e) = e {
                        match food.get(e) {
                            Ok(_) => {
                                if let Some(mut entity) = commands.get_entity(e) {
                                    entity.despawn()
                                }
                            }
                            Err(_) => {
                                chance += 1;
                                continue 'find;
                            }
                        }
                    }
                }

                break Some(random_location);
            };

            let Some(final_child_location) = new_organism_location else {
                //tough
                continue;
            };

            new_organism.insert_at(&mut commands, final_child_location);
            //info!("Child spawned!");
        }
    }
}
