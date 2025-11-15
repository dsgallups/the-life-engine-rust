mod replicator;
use std::collections::HashSet;

use replicator::*;

mod mutator;

mod direction;
pub use direction::*;

use bevy::{math::IVec2, platform::collections::HashMap};
use rand::{
    Rng,
    seq::{IteratorRandom, SliceRandom},
};
use strum::IntoEnumIterator;

use crate::ff_network::{
    CellKind, CellRequirements, Hidden, Input, MutationAction, MutationChances, NeuronTopology,
    Output, genome::mutator::Mutator,
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
    pub fn find_free_spot(&self, rng: &mut impl Rng) -> IVec2 {
        let mut cursor = IVec2::ZERO;

        if self.get(&cursor).is_none() {
            return cursor;
        }

        let mut directions = Direction::random_order(rng);
        let mut checking_cursor = cursor;
        loop {
            for direction in &directions {
                checking_cursor = cursor + direction.vec();
                if self.get(&checking_cursor).is_none() {
                    return checking_cursor;
                }
            }
            cursor = checking_cursor;
            directions.shuffle(rng);
        }
    }

    pub fn num_inputs_outputs(&self) -> (usize, usize) {
        self.map().values().fold((0_usize, 0_usize), |acc, val| {
            (acc.0 + val.inputs.len(), acc.1 + val.outputs.len())
        })
    }

    pub fn get(&self, loc: &IVec2) -> Option<&CellGenome> {
        self.0.get(loc)
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
                    let rand_index = rng.random_range(0..self.cells.0.len());
                    let random_cell_loc = self.cells.0.keys().skip(rand_index).next().unwrap();
                    let loc = *random_cell_loc;
                    self.cells.remove(&loc);
                }
                MutationAction::AddConnection => {
                    Mutator::new(&self.cells, &self.hidden).add_connection(rng);
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
