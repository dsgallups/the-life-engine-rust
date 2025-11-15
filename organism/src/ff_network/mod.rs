mod mutation;
pub use mutation::*;

mod topology;
pub use topology::*;

mod cells;
pub use cells::*;

use bevy::math::IVec2;
use rand::Rng;

pub struct Genome {
    cells: Vec<CellGenome>,
    neurons: Vec<NeuronTopology>,
    mutation: MutationChances,
}
impl Genome {
    pub fn sandbox() -> Self {
        let template = [
            (CellKind::Eye, IVec2::new(0, 0)),
            (CellKind::Launcher, IVec2::new(1, 1)),
            (CellKind::Data, IVec2::new(-1, -1)),
        ];
        let mut inputs: Vec<NeuronTopology> = Vec::new();
        let mut outputs: Vec<NeuronTopology> = Vec::new();

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
                inputs.push(new_input);
            }

            for _ in 0..num_outputs {
                let new_output = NeuronTopology::output();
                cell_outputs.push(new_output.clone());
                outputs.push(new_output);
            }
            let cell = CellGenome {
                kind,
                location,
                inputs: cell_inputs,
                outputs: cell_outputs,
            };

            cells.push(cell);
        }
        for output in outputs.iter_mut() {
            output.set_initial_inputs(inputs.clone());
        }

        let neurons = inputs.into_iter().chain(outputs).collect();

        Self {
            cells,
            neurons,
            mutation: MutationChances::new(20),
        }
    }

    fn scramble(&mut self, rng: &mut impl Rng) {

        //
    }
}

pub struct CellGenome {
    kind: CellKind,
    location: IVec2,
    inputs: Vec<NeuronTopology>,
    outputs: Vec<NeuronTopology>,
}
