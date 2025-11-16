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

#[derive(Debug)]
pub struct Genome {
    pub(crate) cells: CellMap,
    pub(crate) hidden: Vec<NeuronTopology<Hidden>>,
    pub(crate) mutation: MutationChances,
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

    /// Create an empty genome for testing
    #[cfg(test)]
    pub fn empty() -> Self {
        Self {
            cells: CellMap::default(),
            hidden: Vec::new(),
            mutation: MutationChances::new(50),
        }
    }

    /// Create a simple linear genome with one input cell, one hidden, and one output cell
    #[cfg(test)]
    pub fn simple_linear() -> Self {
        let mut genome = Self::empty();

        // Add an Eye cell (input)
        genome.cells.add_cell(IVec2::new(0, 0), CellKind::Eye);

        // Add a Launcher cell (output)
        genome.cells.add_cell(IVec2::new(1, 0), CellKind::Launcher);

        // Connect them through a hidden neuron
        let hidden = NeuronTopology::hidden();

        // Connect input to hidden
        if let Some(eye_cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
            for input in &eye_cell.inputs {
                hidden.add_input(input);
            }
        }

        // Connect hidden to output
        if let Some(launcher_cell) = genome.cells.map_mut().get_mut(&IVec2::new(1, 0)) {
            for output in &launcher_cell.outputs {
                output.add_input(&hidden);
            }
        }

        genome.hidden.push(hidden);
        genome
    }

    /// Create a genome with custom cells at specific positions
    #[cfg(test)]
    pub fn from_cells(cells: Vec<(CellKind, IVec2)>) -> Self {
        let mut genome = Self::empty();

        for (kind, location) in cells {
            genome.cells.add_cell(location, kind);
        }

        genome
    }

    /// Get the number of cells
    #[cfg(test)]
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Get the number of hidden neurons
    #[cfg(test)]
    pub fn hidden_count(&self) -> usize {
        self.hidden.len()
    }

    /// Get cells for testing
    #[cfg(test)]
    pub fn cells(&self) -> &CellMap {
        &self.cells
    }

    /// Get hidden neurons for testing
    #[cfg(test)]
    pub fn hidden_neurons(&self) -> &[NeuronTopology<Hidden>] {
        &self.hidden
    }

    /// Get mutable hidden neurons for testing
    #[cfg(test)]
    pub fn hidden_neurons_mut(&mut self) -> &mut Vec<NeuronTopology<Hidden>> {
        &mut self.hidden
    }
}

#[derive(Debug)]
pub struct CellGenome {
    pub kind: CellKind,
    pub inputs: Vec<NeuronTopology<Input>>,
    pub outputs: Vec<NeuronTopology<Output>>,
}
