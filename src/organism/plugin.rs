use bevy::prelude::*;
use bevy_spatial::SpatialAccess;

use crate::{
    cell::{FoodBundle, KillerPlugin, MouthPlugin, MoverPlugin, ProducerPlugin},
    environment::EnvironmentSettings,
    neighbor::VecExt as _,
    CellTree, GameState,
};

use super::{count, Age, Belly, Organism, StarvedAt};

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
                    age_organism,
                    organism_hunger,
                    starve_organism,
                    reproduce_organism,
                    count::organism_text_update_system,
                )
                    .run_if(in_state(GameState::Playing)),
            );

        app.add_systems(Startup, count::setup_organism_counter);
    }
}

fn age_organism(time: Res<Time>, mut ages: Query<&mut Age>) {
    ages.par_iter_mut().for_each(|mut age| {
        age.0 += time.delta().as_millis() as u64;
    })
}

fn organism_hunger(
    time: Res<Time>,
    settings: Res<EnvironmentSettings>,
    mut organisms: Query<(&mut Belly, &Age, &mut StarvedAt), With<Organism>>,
) {
    organisms
        .par_iter_mut()
        .for_each(|(mut belly, age, mut organism_starved_at)| {
            let current_time = time.elapsed().as_millis() as u64;

            let d_time_last_starved = current_time - organism_starved_at.0;
            if d_time_last_starved >= settings.hunger_tick {
                // based on the age of the organism, we out how much food it should lose
                let age_cost = (age.0 / settings.age_rate) + 1;
                belly.0 = belly.0.saturating_sub(age_cost);
                organism_starved_at.0 = current_time;
            }
        });
}

fn starve_organism(
    par_commands: ParallelCommands,
    organisms: Query<(Entity, &Children, &Belly), With<Organism>>,
    locations: Query<&GlobalTransform>,
) {
    organisms
        .par_iter()
        .for_each(|(organism_entity, children, belly)| {
            if belly.0 == 0 {
                //before the organism dies, we need to turn the children
                //into food :D
                // because it could be killed in the meantime or whatnot, we should just lay down the food here

                par_commands.command_scope(|mut commands| {
                    for child in children.iter() {
                        let location = locations.get(*child).unwrap();
                        commands.spawn(FoodBundle::at(location.translation()));
                    }

                    commands.entity(organism_entity).despawn_recursive();
                });
            }
        })
}

fn reproduce_organism(
    par_commands: ParallelCommands,
    settings: Res<EnvironmentSettings>,
    tree: Res<CellTree>,
    mut organisms: Query<(&GlobalTransform, &mut Belly, &Organism)>,
) {
    if settings
        .max_organisms
        .is_some_and(|max| organisms.iter().count() >= max)
    {
        return;
    }
    organisms
        .par_iter_mut()
        .for_each(|(organism_transform, mut belly, organism)| {
            if organism.ready_to_reproduce(&belly) {
                belly.0 /= 2;
                let Some(new_organism) = organism.reproduce() else {
                    return;
                };

                let organism_location = organism_transform.translation();

                //info!("the organism has created a child!");
                //the organism gets three chances to place within the radius of the parent.
                //otherwise, it dies.
                let mut chance = 0;
                let new_organism_location = 'find: loop {
                    if chance > 3 {
                        break None;
                    }
                    let random_location = organism_location.rand_around(settings.spawn_radius);

                    // must spawn 3 blocks away from anything
                    // todo(dsgallups); hack
                    for (_, e) in
                        tree.within_distance(random_location, organism.radius() as f32 + 1.)
                    {
                        if e.is_some() {
                            chance += 1;
                            continue 'find;
                        }
                    }

                    break Some(random_location);
                };

                let Some(final_child_location) = new_organism_location else {
                    //tough
                    return;
                };

                par_commands.command_scope(|mut commands| {
                    new_organism.insert_at(&mut commands, final_child_location, belly.clone());
                })
                //info!("Child spawned!");
            }
        });
}
