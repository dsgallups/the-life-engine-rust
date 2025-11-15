mod replicator;
use replicator::*;

use bevy::{math::IVec2, platform::collections::HashMap};
use rand::{Rng, seq::IteratorRandom};
use strum::IntoEnumIterator;

use crate::ff_network::{
    CellKind, CellRequirements, Hidden, Input, MutationAction, MutationChances, NeuronTopology,
    Output,
};

#[derive(Default)]
pub struct Cells(HashMap<IVec2, CellGenome>);
impl Cells {
    pub fn with_capacity(cap: usize) -> Self {
        Self(HashMap::with_capacity(cap))
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn map(&self) -> &HashMap<IVec2, CellGenome> {
        &self.0
    }
    pub fn map_mut(&mut self) -> &mut HashMap<IVec2, CellGenome> {
        &mut self.0
    }
    // does not check if a cell is here.
    fn add_cell(&mut self, location: IVec2, cell_kind: CellKind) {
        let mut cell_inputs = Vec::new();
        let mut cell_outputs = Vec::new();
        let CellRequirements {
            num_inputs,
            num_outputs,
        } = cell_kind.requirements();
        for _ in 0..num_inputs {
            let new_input = NeuronTopology::input();
            cell_inputs.push(new_input.clone());
        }

        for _ in 0..num_outputs {
            let new_output = NeuronTopology::output();
            cell_outputs.push(new_output.clone());
        }

        let cell = CellGenome {
            kind: cell_kind,
            inputs: cell_inputs,
            outputs: cell_outputs,
        };
        self.0.insert(location, cell);
    }
    fn remove(&mut self, loc: &IVec2) -> Option<CellGenome> {
        self.0.remove(loc)
    }
}

pub struct Genome {
    cells: Cells,
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
            cells: Cells::default(),
            hidden: Vec::new(),
            mutation: MutationChances::new(20),
        };

        //outputs first
        for (kind, location) in template {
            this.cells.add_cell(location, kind);
        }
        let mut hidden_nodes = Vec::new();

        for cell in this.cells.0.values_mut() {
            for output in cell.outputs.iter_mut() {
                //go 1:1 between hidden and output nodes
                let hidden = NeuronTopology::hidden();
                output.add_input(&hidden);
                hidden_nodes.push(hidden);
            }
        }

        for cell in this.cells.0.values_mut() {
            for hidden_node in hidden_nodes.iter_mut() {
                for input in cell.inputs.iter() {
                    hidden_node.add_input(input);
                }
            }
        }

        this
    }

    fn deep_clone(&self) -> Genome {
        let mut replicator = Replicator::default();
        replicator.process(self)
    }

    fn scramble(&mut self, rng: &mut impl Rng) {
        self.mutation.adjust_mutation_chances(rng);
        let mut mutation_iter = self.mutation.yield_mutations(rng);

        while let Some(action) = mutation_iter.next(rng) {
            match action {
                MutationAction::AddCell => {
                    let new_cell_kind = CellKind::iter().choose(rng).unwrap();
                    self.cells.add_cell(IVec2::ZERO, new_cell_kind);
                }
                MutationAction::DeleteCell => {
                    let rand_index = rng.random_range(0..self.cells.0.len());
                    let (random_cell_loc, _) = self.cells.0.iter().skip(rand_index).next().unwrap();
                    let loc = *random_cell_loc;
                    self.cells.remove(&loc);
                }
                MutationAction::AddConnection => {
                    todo!()
                }
                _ => todo!(),
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
