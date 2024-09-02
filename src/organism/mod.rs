use bevy::prelude::*;
use genome::Genome;
use rand::Rng as _;

use crate::neighbor::VecExt;

use super::cell::OrganismCellType;
pub mod genome;

mod plugin;
pub use plugin::*;

mod reproduction;
use reproduction::*;

#[derive(Copy, Clone, Debug)]
pub enum BrainType {
    Predator,
    Prey,
}

#[derive(Component, Clone, Debug)]
pub struct Organism {
    genome: Genome,
    brain: Option<BrainType>,
    can_move: bool,
    belly: u64,
    tick_born: u64,
    mutation_rate: f64,
    offspring: u64,
}

impl Organism {
    fn new(genome: Genome, belly: u64, tick_born: u64, mutation_rate: f64) -> Self {
        let mut has_producer = false;
        let mut has_eye = false;
        let mut has_mover = false;
        let mut has_killer = false;

        for cell in genome.cells() {
            use OrganismCellType::*;
            match cell.cell_type() {
                Producer => has_producer = true,
                Eye => has_eye = true,
                Mover => has_mover = true,
                Killer => has_killer = true,
                _ => {}
            }
            if has_producer && has_eye && has_mover && has_killer {
                break;
            }
        }

        let brain_type = if has_eye && has_mover {
            if has_killer {
                Some(BrainType::Predator)
            } else {
                Some(BrainType::Prey)
            }
        } else {
            None
        };

        Self {
            genome,
            brain: brain_type,
            can_move: has_mover && !has_producer,
            belly,
            tick_born,
            mutation_rate,
            offspring: 0,
        }
    }
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
    pub fn brain(&self) -> Option<BrainType> {
        self.brain
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

        child.tick_born = current_tick;
        Some(child)
    }

    pub fn first_organism() -> Self {
        Self::new(Genome::first_organism(), 3, 0, 50.)
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
