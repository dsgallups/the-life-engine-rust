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
                        let new_hidden_neuron = self.new_takes_input_neuron(hidden_neuron_inner);
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
            }
            NeuronInputType::Input(input_neuron_inner) => {
                let input_neuron_inner = input_neuron_inner.upgrade()?;
                let input_neuron_inner = input_neuron_inner.lock().unwrap();
                let id = input_neuron_inner.id();

                match self.new_inputs.get(&id) {
                    Some(new_input_neuron) => Some(NeuronInput {
                        input_type: NeuronInputType::Input(Arc::downgrade(&new_input_neuron.inner)),
                        weight: neuron_input.weight,
                    }),
                    None => {
                        //
                        // let new_input_neuron = self.new_takes_input_neuron(input_neuron_inner);
                        // let result = NeuronInput {
                        //     input_type: NeuronInputType::Input(Arc::downgrade(
                        //         &new_input_neuron.inner,
                        //     )),
                        //     weight: neuron_input.weight,
                        // };
                        // self.new_inputs.insert(id, new_input_neuron);
                        todo!()
                        //Some(result)
                    }
                }
            }
        }
    }

    /// creates a new neuron, but doesn't do anything else other than
    /// grab the new inputs.
    fn new_takes_input_neuron<T: TakesInput>(
        &mut self,
        neuron: MutexGuard<'_, super::Inner<T>>,
    ) -> NeuronTopology<T> {
        let mut new_inputs = Vec::new();
        for input in neuron.inputs() {
            if let Some(input) = self.get_inputs(input) {
                new_inputs.push(input);
            }
        }

        let new_inner = Inner {
            id: Uuid::new_v4(),
            n_type: T::new_from_raw_parts(
                new_inputs,
                neuron.n_type.bias(),
                neuron.n_type.activation(),
            ),
        };
        NeuronTopology {
            inner: Arc::new(Mutex::new(new_inner)),
        }
    }
}
