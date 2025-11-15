mod activations;

mod replicator;
use replicator::*;

mod cell_map;
pub use cell_map::*;

mod mutator;

mod direction;
pub use direction::*;

use bevy::math::IVec2;
use rand::{Rng, seq::IteratorRandom};
use strum::IntoEnumIterator;

use crate::ff_network::{
    CellKind, Hidden, Input, MutationAction, MutationChances, NeuronTopology, Output,
    genome::mutator::{ConnectionTask, Mutator, OutputTask},
};

pub struct Genome {
    cells: CellMap,
    hidden: Vec<NeuronTopology<Hidden>>,
    mutation: MutationChances,
}
impl Genome {
    pub fn sandbox() -> Self {
        let template = [
            (CellKind::Eye, IVec2::new(0, 0)),
            (CellKind::Launcher, IVec2::new(1, 1)),
            (CellKind::Data, IVec2::new(-1, -1)),
        ];

        let mut this = Self {
            cells: CellMap::default(),
            hidden: Vec::new(),
            mutation: MutationChances::new(20),
        };

        //outputs first
        for (kind, location) in template {
            this.cells.add_cell(location, kind);
        }
        let mut hidden_nodes = Vec::new();

        for cell in this.cells.map_mut().values_mut() {
            for output in cell.outputs.iter_mut() {
                //go 1:1 between hidden and output nodes
                let hidden = NeuronTopology::hidden();
                output.add_input(&hidden);
                hidden_nodes.push(hidden);
            }
        }

        for cell in this.cells.map_mut().values_mut() {
            for hidden_node in hidden_nodes.iter_mut() {
                for input in cell.inputs.iter() {
                    hidden_node.add_input(input);
                }
            }
        }

        this
    }

    fn deep_clone(&self) -> Genome {
        let replicator = Replicator::new(self);
        replicator.process()
    }

    fn scramble(&mut self, rng: &mut impl Rng) {
        self.mutation.adjust_mutation_chances(rng);
        let mut mutation_iter = self.mutation.yield_mutations(rng);

        while let Some(action) = mutation_iter.next(rng) {
            match action {
                MutationAction::AddCell => {
                    let new_cell_kind = CellKind::iter().choose(rng).unwrap();
                    let new_spot = self.cells.find_free_spot(rng);
                    self.cells.add_cell(new_spot, new_cell_kind);
                }
                MutationAction::DeleteCell => {
                    if self.cells.is_empty() {
                        continue;
                    }
                    let rand_index = rng.random_range(0..self.cells.len());
                    let random_cell_loc = self.cells.map().keys().skip(rand_index).next().unwrap();
                    let loc = *random_cell_loc;
                    self.cells.remove(&loc);
                }
                MutationAction::MutateCell => {
                    if self.cells.is_empty() {
                        continue;
                    }
                    let new_cell_kind = CellKind::iter().choose(rng).unwrap();
                    let rand_index = rng.random_range(0..self.cells.len());
                    let random_cell_loc = self.cells.map().keys().skip(rand_index).next().unwrap();
                    self.cells.add_cell(*random_cell_loc, new_cell_kind);
                }
                MutationAction::AddConnection => {
                    Mutator::new(&self.cells, &mut self.hidden)
                        .with_random_input_and_output(rng, ConnectionTask::Add);
                }
                MutationAction::SplitConnection => {
                    Mutator::new(&self.cells, &mut self.hidden)
                        .with_random_output(rng, OutputTask::Split);
                }
                MutationAction::RemoveNeuron => {
                    if self.hidden.is_empty() {
                        continue;
                    }
                    let random_index = rng.random_range(0..self.hidden.len());
                    self.hidden.swap_remove(random_index);
                }
                MutationAction::MutateWeight => {
                    Mutator::new(&self.cells, &mut self.hidden)
                        .with_random_output(rng, OutputTask::MutateWeight);
                }
                MutationAction::MutateActivation => {
                    Mutator::new(&self.cells, &mut self.hidden)
                        .with_random_output(rng, OutputTask::MutateActivation);
                }
            }
        }

        //
    }
}

pub struct CellGenome {
    kind: CellKind,
    inputs: Vec<NeuronTopology<Input>>,
    outputs: Vec<NeuronTopology<Output>>,
}
