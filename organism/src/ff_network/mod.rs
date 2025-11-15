mod mutation;
pub use mutation::*;

mod neuron;
pub use neuron::*;

mod cells;
pub use cells::*;

mod replicator;
use replicator::*;

use bevy::math::IVec2;
use rand::{Rng, seq::IteratorRandom};
use strum::IntoEnumIterator;

pub struct Genome {
    cells: Vec<CellGenome>,
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

        let mut cells = Vec::new();
        //outputs first
        for (kind, location) in template {
            let mut cell_inputs = Vec::new();
            let mut cell_outputs = Vec::new();
            let CellRequirements {
                num_inputs,
                num_outputs,
            } = kind.requirements();

            for _ in 0..num_inputs {
                let new_input = NeuronTopology::input();
                cell_inputs.push(new_input.clone());
            }

            for _ in 0..num_outputs {
                let new_output = NeuronTopology::output();
                cell_outputs.push(new_output.clone());
            }
            let cell = CellGenome {
                kind,
                location,
                inputs: cell_inputs,
                outputs: cell_outputs,
            };

            cells.push(cell);
        }
        let mut hidden_nodes = Vec::new();

        for cell in cells.iter_mut() {
            for output in cell.outputs.iter_mut() {
                //go 1:1 between hidden and output nodes
                let hidden = NeuronTopology::hidden();
                output.add_input(&hidden);
                hidden_nodes.push(hidden);
            }
        }

        for cell in cells.iter_mut() {
            for hidden_node in hidden_nodes.iter_mut() {
                for input in cell.inputs.iter() {
                    hidden_node.add_input(input);
                }
            }
        }

        Self {
            cells,
            hidden: hidden_nodes,
            mutation: MutationChances::new(20),
        }
    }

    fn deep_clone(&self) -> Genome {
        let mut new_cells = Vec::with_capacity(self.cells.len());
        let mut new_hidden = Vec::with_capacity(self.hidden.len());

        for cell in self.cells.iter() {
            for output in cell.outputs.iter() {
                let n_type = output.lock();
                for input in n_type.inputs() {
                    match input.input_type {
                        NeuronInputType::Hidden(_) => {
                            //todo
                        }
                        NeuronInputType::Input(_) => {
                            //todo
                        }
                    }
                }
            }
        }

        Genome {
            cells: new_cells,
            hidden: new_hidden,
            mutation: self.mutation.clone(),
        }
    }

    fn scramble(&mut self, rng: &mut impl Rng) {
        self.mutation.adjust_mutation_chances(rng);
        let mut mutation_iter = self.mutation.yield_mutations(rng);

        while let Some(action) = mutation_iter.next(rng) {
            match action {
                MutationAction::AddCell => {
                    let new_cell_kind = CellKind::iter().choose(rng).unwrap();
                    todo!()
                }
                MutationAction::DeleteCell => {
                    let rand_index = rng.random_range(0..self.cells.len());
                    self.cells.swap_remove(rand_index);
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
    location: IVec2,
    inputs: Vec<NeuronTopology<Input>>,
    outputs: Vec<NeuronTopology<Output>>,
}
