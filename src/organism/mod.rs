use bevy::prelude::*;
use genome::{CellLocation, Genome, OrganismCell};
use rand::Rng as _;

use crate::neighbor::VecExt;

use super::cell::OrganismCellType;
pub mod genome;

mod plugin;
pub use plugin::*;

mod reproduction;
use reproduction::*;

#[derive(Component, Clone, Debug)]
pub struct Organism {
    genome: Genome,
    ///precomputed
    has_brain: bool,
    ///precomputed
    can_move: bool,
    belly: u64,
    tick_born: u64,
    mutation_rate: f64,
    offspring: u64,
}

impl Organism {
    pub fn ready_to_reproduce(&self) -> bool {
        let can_reproduce_at = self.genome.num_cells() * 3;
        self.belly >= can_reproduce_at as u64
    }

    pub fn num_cells(&self) -> usize {
        self.genome.num_cells()
    }

    /// returns the radius of self given its children
    pub fn radius(&self) -> u32 {
        self.genome.cells().fold(0, |acc, child| {
            acc.max(child.location().x.unsigned_abs())
                .max(child.location().y.unsigned_abs())
        })
    }

    #[allow(dead_code)]
    pub fn has_brain(&self) -> bool {
        self.has_brain
    }

    pub fn can_move(&self) -> bool {
        self.can_move
    }

    pub fn genome(&self) -> &Genome {
        &self.genome
    }

    /// loses half its food.
    pub fn reproduce(&mut self, current_tick: u64) -> Option<Organism> {
        let mut rng = rand::thread_rng();
        self.belly /= 2;

        let mut child = self.clone();
        if rng.gen::<bool>() {
            child.mutation_rate += 1.
        } else {
            child.mutation_rate -= 1.
        };

        let mut mutation_list: Vec<MutationAction> = Vec::new();
        loop {
            let rng_val = rng.gen_range(0..=100);
            if rng_val as f64 <= child.mutation_rate {
                mutation_list.push(MutationAction::rand(&mut rng));
            } else {
                break;
            }
        }

        for mutation in mutation_list {
            match mutation {
                MutationAction::New => self.genome.add_random_cell(&mut rng),
                MutationAction::MutateOrgan => self.genome.mutate_random_cell(&mut rng),
                MutationAction::Delete => {
                    self.genome.delete_random_cell(&mut rng);
                }
            }
        }

        self.offspring += 1;

        if child.num_cells() == 0 {
            return None;
        }
        let mut has_mover = false;
        let mut has_eye = false;
        let mut has_producer = false;
        for cell_type in child.genome().types() {
            match cell_type {
                OrganismCellType::Eye => has_eye = true,
                OrganismCellType::Mover => has_mover = true,
                OrganismCellType::Producer => has_producer = true,
                _ => continue,
            }

            if has_mover && has_eye && has_producer {
                break;
            }
        }
        child.has_brain = has_mover && has_eye;
        child.can_move = has_mover && !has_producer;

        child.tick_born = current_tick;
        Some(child)
    }

    pub fn first_organism() -> Self {
        Self {
            genome: Genome::first_organism(),
            has_brain: false,
            can_move: false,
            belly: 3,
            tick_born: 0,
            mutation_rate: 50.,
            offspring: 0,
        }
    }
    pub fn ate_food(&mut self, amt: u64) {
        self.belly += amt;
    }

    pub fn lost_food(&mut self, amt: u64) {
        self.belly = self.belly.saturating_sub(amt);
        //info!("Organism lost food, belly is at {}", self.belly)
    }

    pub fn tick_born(&self) -> u64 {
        self.tick_born
    }

    pub fn belly(&self) -> u64 {
        self.belly
    }

    pub fn occupying_locations(&self) -> impl Iterator<Item = CellLocation> + '_ {
        self.genome.locations()
    }

    pub fn cells(&self) -> impl Iterator<Item = &OrganismCell> {
        self.genome.cells()
    }

    /// Uses both the ECS and the global positioning hashmap to insert itself.
    pub fn insert_at(self, commands: &mut Commands, location: impl VecExt) {
        /*info!(
            "\nInserting Organism into the world:\nLocation: {:?}\nto insert:{:#?}",
            global_location, self
        );*/
        //need to clone since we move self into the system
        let genome = self.genome.clone();

        commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_translation(location.as_vec3()),
                    ..Default::default()
                },
                self,
            ))
            .with_children(|child_builder| {
                genome.spawn_cells(child_builder);
            });
    }
}
