mod activations;

mod replicator;
use replicator::*;

mod cell_map;
pub use cell_map::*;

pub mod mutator;

pub mod decycler;

mod direction;
pub use direction::*;

use bevy::math::IVec2;
use rand::Rng;

use crate::ff_network::{
    CellKind, Hidden, Input, MutationChances, NeuronTopology, Output, genome::decycler::Cleaner,
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

    pub fn deep_clone(&self) -> Genome {
        let replicator = Replicator::new(self);
        replicator.process()
    }

    pub fn scramble(&mut self, rng: &mut impl Rng) {
        self.mutation.adjust_mutation_chances(rng);
        let mut mutation_iter = self.mutation.yield_mutations(rng);

        while let Some(action) = mutation_iter.next(rng) {
            action.perform(&mut self.cells, &mut self.hidden, rng);
        }

        Cleaner::new(self).clean();
    }
}

pub struct CellGenome {
    kind: CellKind,
    inputs: Vec<NeuronTopology<Input>>,
    outputs: Vec<NeuronTopology<Output>>,
}
