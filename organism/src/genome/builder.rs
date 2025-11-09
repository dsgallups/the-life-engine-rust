use bevy::platform::collections::HashMap;
use nora_neat::{
    neuron::{NeuronInput, NeuronTopology, Topology},
    prelude::{MutationChances, NetworkTopology},
};
use rand::Rng;
use uuid::Uuid;

use crate::{CellGenome, CellTemplate, Genome, PartialCellGenome};

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

        let mut partial_cell_genomes: Vec<PartialCellGenome> =
            Vec::with_capacity(self.cell_templates.capacity());

        let mut input_ir_map: HashMap<usize, Vec<Uuid>> = HashMap::default();
        let mut output_ir_map: HashMap<usize, Vec<Uuid>> = HashMap::default();

        let mut input_neurons = Vec::new();
        let mut output_neurons = Vec::new();

        for cell_template in self.cell_templates {
            let p_net_template = cell_template.template();

            let partial_cell_genome = cell_template.into_genome();
            partial_cell_genomes.push(partial_cell_genome);
            let id = partial_cell_genomes.len() - 1;

            for _ in 0..p_net_template.input_junctions() {
                let topology_id = Uuid::new_v4();
                let neuron_topology = NeuronTopology::input(topology_id);

                input_neurons.push(neuron_topology);
                let indices = input_ir_map.entry(id).or_default();
                indices.push(topology_id);
            }

            for _ in 0..p_net_template.output_junctions() {
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

        let mut cells = Vec::with_capacity(partial_cell_genomes.capacity());

        // at this point, we have input mapping (uuid to indices into the topology)
        // and partial cells. We just need to finish the owl now.
        //from the network topology, we will map the ids into indices into this topology
        for (ir_id, partial_cell_genome) in partial_cell_genomes.into_iter().enumerate() {
            let mut cell_genome = CellGenome::new(
                partial_cell_genome.id(),
                partial_cell_genome.kind().cell_details(),
                partial_cell_genome.location(),
            );

            if let Some(inputs) = input_ir_map.remove(&ir_id) {
                let mut cell_inputs = Vec::with_capacity(inputs.len());
                for input_id in inputs {
                    let position = network_topology
                        .neurons()
                        .iter()
                        .position(|neuron| neuron.id() == input_id)
                        .unwrap();
                    cell_inputs.push(position);
                }
                cell_genome.set_inputs(cell_inputs);
            }

            if let Some(outputs) = output_ir_map.remove(&ir_id) {
                let mut cell_outputs = Vec::with_capacity(outputs.len());
                for output_id in outputs {
                    let position = network_topology
                        .neurons()
                        .iter()
                        .position(|neuron| neuron.id() == output_id)
                        .unwrap();
                    cell_outputs.push(position);
                }
                cell_genome.set_outputs(cell_outputs);
            }
            cells.push(cell_genome);
        }

        Genome {
            cells,
            network_topology,
        }
    }
}
