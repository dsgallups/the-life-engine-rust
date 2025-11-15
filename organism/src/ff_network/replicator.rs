use std::sync::{Arc, Mutex, MutexGuard};

use bevy::platform::collections::HashMap;
use uuid::Uuid;

use crate::ff_network::{
    CellGenome, Genome, Hidden, Inner, Input, NeuronInput, NeuronInputType, NeuronTopology, Output,
    TakesInput,
};

#[derive(Default)]
pub struct Replicator {
    new_outputs: HashMap<Uuid, NeuronTopology<Output>>,
    new_hidden: HashMap<Uuid, NeuronTopology<Hidden>>,
    new_inputs: HashMap<Uuid, NeuronTopology<Input>>,
}
impl Replicator {
    pub fn process(&mut self, genome: &Genome) {
        for cell in genome.cells.iter() {
            self.replicate_cell(cell);
        }

        todo!()
    }

    fn replicate_cell(&mut self, cell: &CellGenome) {
        for output in cell.outputs.iter() {
            //let new_inputs = self.get_inputs(output.lock().inputs());
            //match
        }
    }
    fn get_inputs(&mut self, neuron_input: &NeuronInput) -> Option<NeuronInput> {
        match &neuron_input.input_type {
            NeuronInputType::Hidden(hidden_neuron_inner) => {
                let hidden_neuron_inner = hidden_neuron_inner.upgrade()?;
                let hidden_neuron_inner = hidden_neuron_inner.lock().unwrap();
                let id = hidden_neuron_inner.id();

                match self.new_hidden.get(&id) {
                    Some(new_hidden_neuron) => Some(NeuronInput {
                        input_type: NeuronInputType::Hidden(Arc::downgrade(
                            &new_hidden_neuron.inner,
                        )),
                        weight: neuron_input.weight,
                    }),
                    None => {
                        let new_hidden_neuron = self.new_neuron(hidden_neuron_inner);
                        let result = NeuronInput {
                            input_type: NeuronInputType::Hidden(Arc::downgrade(
                                &new_hidden_neuron.inner,
                            )),
                            weight: neuron_input.weight,
                        };
                        self.new_hidden.insert(id, new_hidden_neuron);
                        Some(result)
                    }
                }

                //instead,

                //todo
            }
            NeuronInputType::Input(input_neuron_inner) => {
                todo!()
            }
        }
    }

    /// creates a new neuron, but doesn't do anything else other than
    /// grab the new inputs.
    fn new_neuron(
        &mut self,
        neuron: MutexGuard<'_, super::Inner<Hidden>>,
    ) -> NeuronTopology<Hidden> {
        let mut new_inputs = Vec::new();
        for input in neuron.inputs() {
            if let Some(input) = self.get_inputs(input) {
                new_inputs.push(input);
            }
        }

        let new_inner = Inner {
            id: Uuid::new_v4(),
            n_type: Hidden {
                inputs: new_inputs,
                bias: neuron.n_type.bias,
                activation: neuron.n_type.activation,
            },
        };
        NeuronTopology {
            inner: Arc::new(Mutex::new(new_inner)),
        }
    }
}
