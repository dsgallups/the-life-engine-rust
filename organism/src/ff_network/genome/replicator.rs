use std::sync::{Arc, Mutex};

use bevy::platform::collections::HashMap;
use uuid::Uuid;

use crate::ff_network::{
    CellGenome, CellMap, Genome, Hidden, Input, NeuronInput, NeuronInputType, NeuronTopology,
    TakesInput, TopologyNeuron,
};

pub struct Replicator<'a> {
    genome: &'a Genome,
    new_hidden: HashMap<Uuid, NeuronTopology<Hidden>>,
    new_inputs: HashMap<Uuid, NeuronTopology<Input>>,
}
impl<'a> Replicator<'a> {
    pub fn new(genome: &'a Genome) -> Self {
        let hidden = genome.hidden.len();
        let inputs = genome
            .cells
            .map()
            .values()
            .map(|cell| cell.inputs.len())
            .sum::<usize>();

        Self {
            new_hidden: HashMap::with_capacity(hidden),
            new_inputs: HashMap::with_capacity(inputs),
            genome,
        }
    }
    pub fn process(mut self) -> Genome {
        let mut interior_output_map = HashMap::with_capacity(self.genome.cells.len());
        // process all outputs first to populate all the maps
        for (i, cell) in self.genome.cells.map().values().enumerate() {
            let mut new_outputs = Vec::with_capacity(cell.outputs.len());
            for output in cell.outputs.iter() {
                new_outputs.push(self.new_takes_input_neuron(&*output.lock()));
            }
            interior_output_map.insert(i, new_outputs);
        }

        let mut new_cells: CellMap = CellMap::with_capacity(self.genome.cells.len());

        for (i, (cell_loc, cell)) in self.genome.cells.map().iter().enumerate() {
            let outputs = interior_output_map.remove(&i).unwrap();
            let mut inputs = Vec::with_capacity(cell.inputs.len());

            for input in cell.inputs.iter() {
                let id = input.id();
                let input = self.new_inputs.remove(&id).unwrap();
                inputs.push(input);
            }
            new_cells.map_mut().insert(
                *cell_loc,
                CellGenome {
                    kind: cell.kind,
                    inputs,
                    outputs,
                },
            );
        }

        let new_hidden = self.new_hidden.drain().map(|(_, v)| v).collect();

        Genome {
            cells: new_cells,
            hidden: new_hidden,
            mutation: self.genome.mutation.clone(),
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
                        let new_hidden_neuron = self.new_takes_input_neuron(&*hidden_neuron_inner);
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
                        let new_input_neuron = NeuronTopology::input();
                        let result = NeuronInput {
                            input_type: NeuronInputType::Input(Arc::downgrade(
                                &new_input_neuron.inner,
                            )),
                            weight: neuron_input.weight,
                        };
                        self.new_inputs.insert(id, new_input_neuron);
                        Some(result)
                    }
                }
            }
        }
    }

    /// creates a new neuron, but doesn't do anything else other than
    /// grab the new inputs.
    fn new_takes_input_neuron<T: TakesInput>(&mut self, neuron: &T) -> NeuronTopology<T> {
        let mut new_inputs = Vec::new();
        for input in neuron.inputs() {
            if let Some(input) = self.get_inputs(input) {
                new_inputs.push(input);
            }
        }
        let new_t = T::new_from_raw_parts(new_inputs, neuron.bias(), neuron.activation());

        NeuronTopology {
            inner: Arc::new(Mutex::new(new_t)),
        }
    }
}
