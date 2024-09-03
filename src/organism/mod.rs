use bevy::prelude::*;
use genome::Genome;

use crate::neighbor::VecExt;

use super::cell::OrganismCellType;
pub mod genome;

mod plugin;
pub use plugin::*;

mod reproduction;
use reproduction::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BrainType {
    Predator,
    Prey,
}

#[derive(Component, Clone, Debug)]
pub struct Organism {
    genome: Genome,
    brain: Option<BrainType>,
    can_move: bool,
    offspring: u64,
    /// in millis
    last_starved: u64,
    can_reproduce_at: u64,
}

impl Organism {
    fn new(genome: Genome) -> Self {
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

        //let can_reproduce_at = (genome.num_cells() * 3).max(6) as u64;
        let can_reproduce_at = (genome.num_cells() * 3) as u64;

        Self {
            genome,
            brain: brain_type,
            can_move: has_mover && !has_producer,
            offspring: 0,
            last_starved: 0,
            can_reproduce_at,
        }
    }
    pub fn ready_to_reproduce(&self, belly: &Belly) -> bool {
        belly.food() >= self.can_reproduce_at
    }

    /// returns the number of cells this organism takes up based on its genome
    pub fn size(&self) -> usize {
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

    pub fn reproduce(&self) -> Option<Organism> {
        //always lose half of one's belly
        let child_genome = self.genome.reproduce();
        if child_genome.num_cells() == 0 {
            return None;
        }
        //self.offspring += 1;
        Some(Self::new(child_genome))
    }

    pub fn first_organism() -> Self {
        Self::new(Genome::first_organism())
    }

    /// returns the millisecond last starved
    pub fn time_last_starved(&self) -> u64 {
        self.last_starved
    }

    /// Uses both the ECS and the global positioning hashmap to insert itself.
    pub fn insert_at(self, commands: &mut Commands, location: impl VecExt, belly: Belly) {
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
                belly,
                Age::default(),
            ))
            .with_children(|child_builder| {
                genome.spawn_cells(child_builder);
            });
    }
}
