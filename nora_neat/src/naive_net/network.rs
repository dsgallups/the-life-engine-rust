use rayon::iter::{IndexedParallelIterator as _, IntoParallelRefIterator, ParallelIterator as _};

use crate::{
    naive_net::neuron::{NaiveNeuron, to_neuron},
    prelude::*,
};

pub struct NaiveNetwork {
    // contains all neurons
    neurons: Vec<NaiveNeuron>,
    // contains the input neurons. cloned arc of neurons in neurons
    input_layer: Vec<NaiveNeuron>,
    // contains the output neurons. cloned arc of neurons in neurons
    output_layer: Vec<NaiveNeuron>,
}

impl NaiveNetwork {
    /// Create a network from raw components.
    ///
    /// This is a low-level constructor that assumes the provided components
    /// are correctly structured. The input and output layer vectors should
    /// contain references to neurons that also exist in the main neurons vector.
    pub fn from_raw_parts(
        neurons: Vec<NaiveNeuron>,
        input_layer: Vec<NaiveNeuron>,
        output_layer: Vec<NaiveNeuron>,
    ) -> Self {
        Self {
            neurons,
            input_layer,
            output_layer,
        }
    }

    /// Create an executable network from a topology representation.
    ///
    /// This method converts a `NetworkTopology` (which represents the
    /// structure and evolution parameters) into a `SimplePolyNetwork` that
    /// can perform inference.
    pub fn from_topology(topology: &NetworkTopology) -> Self {
        let mut neurons: Vec<NaiveNeuron> = Vec::with_capacity(topology.neurons().len());
        let mut input_layer: Vec<NaiveNeuron> = Vec::new();
        let mut output_layer: Vec<NaiveNeuron> = Vec::new();

        for neuron_replicant in topology.neurons() {
            let neuron = neuron_replicant.read().unwrap();

            to_neuron(&neuron, &mut neurons);
            let neuron = neurons.iter().find(|n| n.id() == neuron.id()).unwrap();

            let neuron_read = neuron.inner().read().unwrap();

            if neuron_read.is_input() {
                input_layer.push(neuron.clone());
            }
            if neuron_read.is_output() {
                output_layer.push(neuron.clone());
            }
        }

        NaiveNetwork::from_raw_parts(neurons, input_layer, output_layer)
    }

    /// Perform a forward pass through the network with the given inputs.
    ///
    /// This method:
    /// 1. Resets all neuron states to prepare for a fresh computation
    /// 2. Sets the input values on input neurons
    /// 3. Propagates values through the network
    /// 4. Returns the outputs from output neurons
    ///
    /// # Arguments
    /// * `inputs` - Slice of input values. Length should match the number of input neurons.
    ///
    /// # Note
    /// If there are more inputs than input neurons, extra inputs are ignored.
    /// If there are fewer inputs than input neurons, the remaining neurons
    /// will have their state set to 0.
    pub fn predict(&self, inputs: &[f32]) -> impl Iterator<Item = f32> {
        // reset all states first
        self.neurons.par_iter().for_each(|neuron| {
            let mut neuron = neuron.inner().write().unwrap();
            neuron.flush_state();
        });
        inputs.par_iter().enumerate().for_each(|(index, value)| {
            let Some(nw) = self.input_layer.get(index) else {
                //sim
                return;
                //panic!("couldn't flush i {}", index);
            };
            let mut nw = nw.inner().write().unwrap();
            nw.override_state(*value);
        });

        let outputs = self
            .output_layer
            .par_iter()
            .fold(Vec::new, |mut values, neuron| {
                let mut neuron = neuron.inner().write().unwrap();

                values.push(neuron.activate());

                values
            })
            .collect_vec_list();

        outputs
            .into_iter()
            .flat_map(|outer_vec| outer_vec.into_iter())
            .flat_map(|inner_vec| inner_vec.into_iter())
    }

    /// Generate a human-readable summary of the network's structure.
    ///
    /// # Returns
    /// A formatted string describing the network's neuron counts.
    pub fn summarize(&self) -> String {
        format!(
            "Network with \n{} total nodes\n{} input nodes\n{} output nodes",
            self.num_nodes(),
            self.num_inputs(),
            self.num_outputs()
        )
    }

    /// Get the total number of neurons in the network.
    ///
    /// This includes input, hidden, and output neurons.
    pub fn num_nodes(&self) -> usize {
        self.neurons.len()
    }

    /// Get the number of input neurons.
    ///
    /// This determines how many input values the network expects.
    pub fn num_inputs(&self) -> usize {
        self.input_layer.len()
    }

    /// Get the number of output neurons.
    ///
    /// This determines how many output values the network produces.
    pub fn num_outputs(&self) -> usize {
        self.output_layer.len()
    }

    /// Generate a detailed debug representation of the network structure.
    ///
    /// This method provides a comprehensive view of:
    /// - All neurons with their IDs and types
    /// - Connection patterns between neurons
    /// - Layer organization
    ///
    /// The output format shows neuron indices in parentheses and connection
    /// targets in square brackets.
    ///
    /// # Returns
    /// A formatted string with detailed network structure information.
    ///
    /// # Example Output
    /// ```text
    /// neurons:
    /// ((0) a1b2c3[input]: N/A)
    /// ((1) d4e5f6[hidden]: [(0)])
    /// ((2) g7h8i9[output]: [(1)])
    ///
    /// input_layer:
    /// ((0) a1b2c3[input]: N/A)
    ///
    /// output layer:
    /// ((0) g7h8i9[output]: [(1)])
    /// ```
    pub fn debug_str(&self) -> String {
        let mut str = "neurons: \n".to_string();
        for (neuron_index, neuron) in self.neurons.iter().enumerate() {
            let neuron = neuron.inner().read().unwrap();
            str.push_str(&format!(
                "\n(({}) {}[{}]: ",
                neuron_index,
                neuron.id_short(),
                neuron.neuron_type()
            ));
            match neuron.props() {
                Some(props) => {
                    str.push('[');
                    for input in props.inputs() {
                        let n = input.input().handle().inner().read().unwrap();

                        let loc = self
                            .neurons
                            .iter()
                            .position(|neuron| neuron.id() == n.id())
                            .unwrap();

                        str.push_str(&format!("({})", loc));
                    }
                    str.push(']')
                }

                None => {
                    str.push_str("N/A");
                }
            }

            str.push(')');
        }

        str.push_str("\n\ninput_layer:");

        for (neuron_index, neuron) in self.input_layer.iter().enumerate() {
            let neuron = neuron.inner().read().unwrap();
            str.push_str(&format!(
                "\n(({}) {}[{}]: ",
                neuron_index,
                neuron.id_short(),
                neuron.neuron_type()
            ));
            match neuron.props() {
                Some(props) => {
                    str.push('[');
                    for input in props.inputs() {
                        let n = input.input().handle().inner().read().unwrap();

                        let loc = self
                            .neurons
                            .iter()
                            .position(|neuron| neuron.id() == n.id())
                            .unwrap();

                        str.push_str(&format!("({})", loc));
                    }
                    str.push(']')
                }

                None => {
                    str.push_str("N/A");
                }
            }

            str.push(')');
        }

        str.push_str("\n\noutput layer:");

        for (neuron_index, neuron) in self.output_layer.iter().enumerate() {
            let neuron = neuron.inner().read().unwrap();
            str.push_str(&format!(
                "\n(({}) {}[{}]: ",
                neuron_index,
                neuron.id_short(),
                neuron.neuron_type()
            ));
            match neuron.props() {
                Some(props) => {
                    str.push('[');
                    for input in props.inputs() {
                        let n = input.input().handle().inner().read().unwrap();

                        let loc = self
                            .neurons
                            .iter()
                            .position(|neuron| neuron.id() == n.id())
                            .unwrap();

                        str.push_str(&format!("({})", loc));
                    }
                    str.push(']')
                }

                None => {
                    str.push_str("N/A");
                }
            }

            str.push(')');
        }

        str
    }
}
