use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use bevy::prelude::*;
use uuid::Uuid;

mod neuron;
use neuron::*;

use crate::{
    CellKind,
    genome::{Genome, NeuronInputType, NeuronTopology, TakesInput},
};

#[derive(Component)]
pub struct Cell {
    kind: CellKind,
    inputs: Vec<CpuNeuron>,
    outputs: Vec<CpuNeuron>,
}

impl Cell {
    pub fn kind(&self) -> CellKind {
        self.kind
    }
    pub fn set(&self, index: usize, value: f32) {
        let input = &self.inputs[index];
        let mut inner = input.inner.write().unwrap();

        inner.value = Some(value);
    }
    pub fn get(&self, index: usize) -> f32 {
        let output = &self.outputs[index];
        output.process()
    }
}

pub struct CpuNetwork {
    pub cells: HashMap<IVec2, Cell>,
}

impl CpuNetwork {
    pub fn new(genome: &Genome) -> Self {
        let mut neuron_bank = HashMap::new();

        let mut output_map = HashMap::new();

        for (cell, cell_genome) in genome.cells().map() {
            let mut new_outputs = Vec::new();

            for output_neuron in &cell_genome.outputs {
                new_outputs.push(process_topology(output_neuron, &mut neuron_bank));
            }
            output_map.insert(*cell, new_outputs);
        }

        let mut cells = HashMap::with_capacity(genome.cells().len());

        for (cell_location, cell_genome) in genome.cells().map() {
            let mut new_inputs = Vec::new();

            for input_neuron in &cell_genome.inputs {
                let id = input_neuron.id();
                match neuron_bank.remove(&id) {
                    Some(neuron) => {
                        new_inputs.push(neuron);
                    }
                    None => new_inputs.push(CpuNeuron::input()),
                }
            }
            let outputs = output_map.remove(cell_location).unwrap();

            cells.insert(
                *cell_location,
                Cell {
                    kind: cell_genome.kind,
                    inputs: new_inputs,
                    outputs,
                },
            );
        }
        Self { cells }
    }
}

fn process_topology<T: TakesInput>(
    neuron: &NeuronTopology<T>,
    neurons: &mut HashMap<Uuid, CpuNeuron>,
) -> CpuNeuron {
    let read = neuron.read();

    let mut cpu_inputs = Vec::new();

    for input in read.inputs() {
        match &input.input_type {
            NeuronInputType::Hidden(hidden) => {
                let Some(hidden) = hidden.upgrade() else {
                    continue;
                };
                let id = hidden.id();
                match neurons.get(&id) {
                    Some(neuron) => {
                        cpu_inputs.push((neuron.clone(), input.weight));
                    }
                    None => {
                        let new_neuron = process_topology(&hidden, neurons);

                        cpu_inputs.push((new_neuron.clone(), input.weight));
                        neurons.insert(id, new_neuron);
                    }
                }
            }
            NeuronInputType::Input(input_neuron) => {
                let Some(input_neuron) = input_neuron.upgrade() else {
                    continue;
                };
                let id = input_neuron.id();
                match neurons.get(&id) {
                    Some(neuron) => {
                        cpu_inputs.push((neuron.clone(), input.weight));
                    }
                    None => {
                        let new_neuron = CpuNeuron::input();
                        cpu_inputs.push((new_neuron.clone(), input.weight));
                        neurons.insert(id, new_neuron);
                    }
                }
                //todo
            }
        }
    }
    let cpu_neuron_inputs = CpuNeuronInputs {
        inputs: cpu_inputs,
        bias: read.bias(),
        activation: read.activation(),
    };
    let inner = CpuNeuronInner {
        inputs: Some(cpu_neuron_inputs),
        value: None,
    };

    CpuNeuron {
        inner: Arc::new(RwLock::new(inner)),
    }
}
