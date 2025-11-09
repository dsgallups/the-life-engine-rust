use bevy::platform::collections::HashMap;
use nora_neat::{
    neuron::{NeuronInput, NeuronTopology, Topology},
    prelude::{MutationChances, NetworkTopology},
};
use rand::Rng;
use uuid::Uuid;

use crate::{CellTemplate, Genome};

pub struct GenomeBuilder {
    cell_templates: Vec<CellTemplate>,
}

impl GenomeBuilder {
    pub fn new(templates: impl IntoIterator<Item = CellTemplate>) -> Self {
        Self {
            cell_templates: templates.into_iter().collect(),
        }
    }

    pub fn build(self, rng: &mut impl Rng) -> Genome {
        // these input maps will first map to neuron topology uuids.
        //
        // After we insert the neuron topology ids, we will then grab the uuids out of the topology.
        //
        // This means that outputs can potentially be intermediary nodes. in the future.
        let mut input_ir_map: HashMap<Uuid, Vec<Uuid>> = HashMap::default();
        let mut output_ir_map: HashMap<Uuid, Vec<Uuid>> = HashMap::default();

        let mut input_neurons = Vec::new();
        let mut output_neurons = Vec::new();

        for cell_template in self.cell_templates {
            let p_net_template = cell_template.template();
            let id = Uuid::new_v4();

            for i in 0..p_net_template.input_junctions() {
                let topology_id = Uuid::new_v4();
                let neuron_topology = NeuronTopology::input(topology_id);

                input_neurons.push(neuron_topology);
                let indices = input_ir_map.entry(id).or_default();
                indices.push(topology_id);
            }

            for i in 0..p_net_template.output_junctions() {
                let topology_id = Uuid::new_v4();
                let output_topology = NeuronTopology::output(topology_id, vec![], rng);

                output_neurons.push(output_topology);
                let indices = output_ir_map.entry(id).or_default();
                indices.push(topology_id);
            }
        }

        for output_neuron in &output_neurons {
            let inputs_for_output = input_neurons
                .iter()
                .map(|input| NeuronInput::new_rand(Topology::new(input), rng))
                .collect::<Vec<_>>();

            let mut lock = output_neuron.write();
            let props = lock.props_mut().unwrap();
            props.set_inputs(inputs_for_output);
        }

        let mutation_chances = MutationChances::new(10);
        let network_topology = NetworkTopology::from_raw_parts(
            input_neurons.into_iter().chain(output_neurons),
            mutation_chances,
        );

        //from the network topology, we will map the associated UUIDs into indices into this topology

        let mut input_map: HashMap<Uuid, Vec<usize>> =
            HashMap::with_capacity(input_ir_map.capacity());
        let mut output_map: HashMap<Uuid, Vec<usize>> =
            HashMap::with_capacity(output_ir_map.capacity());

        for (input_id, input_neuron_ids) in input_ir_map {
            let mut ids = Vec::with_capacity(input_neuron_ids.capacity());
            for id in input_neuron_ids {
                let position = network_topology
                    .neurons()
                    .iter()
                    .position(|neuron| neuron.id() == id)
                    .unwrap();
                ids.push(position)
            }
            input_map.insert(input_id, ids);
        }

        for (output_id, output_neuron_ids) in output_ir_map {
            let mut ids = Vec::with_capacity(output_neuron_ids.capacity());
            for id in output_neuron_ids {
                let position = network_topology
                    .neurons()
                    .iter()
                    .position(|neuron| neuron.id() == id)
                    .unwrap();
                ids.push(position)
            }
            output_map.insert(output_id, ids);
        }

        todo!()
    }
}
