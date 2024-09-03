use bevy::prelude::*;
use bevy_spatial::SpatialAccess;

use crate::{
    cell::{FoodBundle, KillerPlugin, MouthPlugin, MoverPlugin, ProducerPlugin},
    environment::EnvironmentSettings,
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

#[derive(Component, Default)]
pub struct Age {
    ticks_alive: u64,
}
impl Age {
    pub fn age(&self) -> u64 {
        self.ticks_alive
    }
    pub fn tick(&mut self) {
        self.ticks_alive += 1;
    }
}

#[derive(Component, Default, Clone)]
pub struct Belly(u64);

impl Belly {
    pub fn new(amt: u64) -> Self {
        Self(amt)
    }
    pub fn ate_food(&mut self, amt: u64) {
        self.0 += amt;
    }
    pub fn food(&self) -> u64 {
        self.0
    }
    pub fn lost_food(&mut self, amt: u64) {
        self.0 = self.0.saturating_sub(amt);
    }
}

fn starve_organism(
    mut commands: Commands,
    settings: Res<EnvironmentSettings>,
    mut organisms: Query<(Entity, &Children, &mut Belly, &mut Age), With<Organism>>,
    locations: Query<&GlobalTransform>,
) {
    for (organism_entity, children, mut organism_belly, mut organism_age) in &mut organisms {
        organism_age.tick();
        if organism_age.age() % settings.hunger_tick == 0 {
            // based on the age of the organism, we out how much food it should lose
            let age_cost = (organism_age.age() / settings.age_rate) + 1;
            organism_belly.lost_food(age_cost);
        }

        if organism_belly.food() == 0 {
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
    tree: Res<CellTree>,
    mut organisms: Query<(&GlobalTransform, &Organism, &mut Belly)>,
) {
    if settings
        .max_organisms
        .is_some_and(|max| organisms.iter().count() >= max)
    {
        return;
    }

    for (organism_transform, organism, mut belly) in &mut organisms {
        if organism.ready_to_reproduce(&belly) {
            belly.0 /= 2;
            let Some(new_organism) = organism.reproduce() else {
                continue;
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
                for (_, e) in tree.within_distance(random_location, organism.radius() as f32 + 1.) {
                    if e.is_some() {
                        chance += 1;
                        continue 'find;
                    }
                }

                break Some(random_location);
            };

            let Some(final_child_location) = new_organism_location else {
                //tough
                continue;
            };

            new_organism.insert_at(&mut commands, final_child_location, belly.clone());
            //info!("Child spawned!");
        }
    }
}
