use bevy::{math::IVec2, platform::collections::HashMap};
use rand::{Rng, seq::SliceRandom};

use crate::{
    cell::{CellGenome, CellKind, CellRequirements},
    genome::{Direction, NeuronTopology},
};

#[derive(Default, Clone, Debug)]
pub struct CellMap(HashMap<IVec2, CellGenome>);
impl CellMap {
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
    pub fn add_cell(
        &mut self,
        location: IVec2,
        cell_kind: CellKind,
        rng: &mut impl Rng,
    ) -> Option<CellGenome> {
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
            let new_output = NeuronTopology::output(rng);
            cell_outputs.push(new_output.clone());
        }

        let cell = CellGenome {
            kind: cell_kind,
            inputs: cell_inputs,
            outputs: cell_outputs,
        };
        self.0.insert(location, cell)
    }
    pub fn remove(&mut self, loc: &IVec2) -> Option<CellGenome> {
        self.0.remove(loc)
    }
}
