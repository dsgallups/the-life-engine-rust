use rand::Rng;

use crate::ff_network::{CellMap, Hidden, NeuronTopology};

pub struct Mutator<'a> {
    cells: &'a CellMap,
    hidden: &'a [NeuronTopology<Hidden>],
}

impl<'a> Mutator<'a> {
    pub fn new(cells: &'a CellMap, hidden: &'a [NeuronTopology<Hidden>]) -> Self {
        Self { cells, hidden }
    }

    pub fn add_connection(&self, rng: &mut impl Rng) {
        let (num_inputs, num_outputs) = self.cells.num_inputs_outputs();
        let neurons_capable_of_taking_input = num_outputs + self.hidden.len();
        let neurons_capable_of_being_input = num_inputs + self.hidden.len();

        let input_neuron = rng.random_range(0..neurons_capable_of_being_input);

        let output_neuron = rng.random_range(0..neurons_capable_of_taking_input);
        //num_outputs = 2, num_hidden = 3;
        // 5
        // rand_index picks 1
        // 1 = num_outputs[1] (last one)
        //
        // if 2 >= 2
        // if 1 >= 2
        let input_is_hidden = input_neuron >= num_inputs;
        let output_is_hidden = output_neuron >= num_outputs;

        match (input_is_hidden, output_is_hidden) {
            (true, true) => {
                let input_neuron_i = input_neuron - num_inputs;
                let output_neuron_i = output_neuron - num_outputs;
                if input_neuron_i == output_neuron_i {
                    return;
                }
                let input_neuron = &self.hidden[input_neuron_i];
                let output_neuron = &self.hidden[output_neuron_i];
                output_neuron.add_input(&*input_neuron);
            }
            (true, false) => {
                let input_neuron_i = input_neuron - num_inputs;
                let output_neuron_i = output_neuron;
                let mut i = 0;
                'outer: for cell in self.cells.map().values() {
                    for output_neuron in cell.outputs.iter() {
                        if i == output_neuron_i {
                            let input_neuron = &self.hidden[input_neuron_i];
                            output_neuron.add_input(&*input_neuron);
                            break 'outer;
                        }
                        i += 1;
                    }
                }
            }
            (false, true) => {
                let input_neuron_i = input_neuron;
                let output_neuron_i = output_neuron - num_outputs;
                let mut i = 0;
                'outer: for cell in self.cells.map().values() {
                    for input_neuron in cell.inputs.iter() {
                        if i == input_neuron_i {
                            let output_neuron = &self.hidden[output_neuron_i];
                            output_neuron.add_input(&*input_neuron);
                            break 'outer;
                        }
                        i += 1;
                    }
                }
            }
            (false, false) => {
                let input_neuron_i = input_neuron;
                let output_neuron_i = output_neuron;

                let mut found_input_neuron = None;
                let mut found_output_neuron = None;

                let mut input_index = 0;
                let mut output_index = 0;
                for cell in self.cells.map().values() {
                    if found_input_neuron.is_some() && found_output_neuron.is_some() {
                        break;
                    }
                    if found_input_neuron.is_none() {
                        for input_neuron in cell.inputs.iter() {
                            if input_index == input_neuron_i {
                                found_input_neuron = Some(input_neuron);
                                break;
                            }
                            input_index += 1;
                        }
                    }
                    if found_output_neuron.is_none() {
                        for output_neuron in cell.outputs.iter() {
                            if output_index == output_neuron_i {
                                found_output_neuron = Some(output_neuron);
                                break;
                            }
                            output_index += 1;
                        }
                    }
                }
                if let Some(input_neuron) = found_input_neuron
                    && let Some(output_neuron) = found_output_neuron
                {
                    output_neuron.add_input(input_neuron);
                }
            }
        }
    }
}
