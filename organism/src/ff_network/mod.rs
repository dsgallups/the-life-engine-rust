mod mutation;
pub use mutation::*;

mod neuron;
pub use neuron::*;

mod cells;
pub use cells::*;

use bevy::math::IVec2;
use rand::Rng;

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
            //go 1:m for input and hidden nodes
            //

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

    fn scramble(&mut self, rng: &mut impl Rng) {
        self.mutation.adjust_mutation_chances(rng);

        for action in self.mutation.yield_mutations(rng) {
            match action {
                MutationAction::AddCell => {
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
