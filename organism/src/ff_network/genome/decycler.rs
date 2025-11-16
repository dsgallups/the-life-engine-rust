use std::collections::HashSet;

use bevy::platform::collections::HashMap;
use uuid::Uuid;

use crate::ff_network::{Genome, NeuronInputType, NeuronTopology, TakesInput};

pub struct Cleaner<'a> {
    // stack: HashSet<Uuid>,
    // visited: HashSet<Uuid>,
    genome: &'a mut Genome,
}
impl<'a> Cleaner<'a> {
    pub fn new(genome: &'a mut Genome) -> Self {
        Self {
            // stack: HashSet::new(),
            // visited: HashSet::new(),
            genome,
        }
    }
    pub fn remove_dead_connections(&mut self) {
        for cell in self.genome.cells.map_mut().values_mut() {
            for output in cell.outputs.iter_mut() {
                output.with_mut(|val| val.inputs_mut().retain(|input| input.is_alive()))
            }
        }

        for hidden_node in self.genome.hidden.iter_mut() {
            hidden_node.with_mut(|h| h.inputs_mut().retain(|input| input.is_alive()))
        }
    }

    pub fn clean(&mut self) {
        self.remove_dead_connections();
        self.decycle();
    }

    pub fn decycle(&mut self) {
        let mut stack = HashSet::new();
        let mut visited = HashSet::new();

        loop {
            let mut remove_queue = RemoveFrom::default();

            'cells: for cell in self.genome.cells.map_mut().values_mut() {
                for output in &cell.outputs {
                    let to_remove = dfs(output, &mut stack, &mut visited);

                    if !to_remove.0.is_empty() {
                        remove_queue = to_remove;
                        break 'cells;
                    }
                }
            }

            if remove_queue.0.is_empty() {
                break;
            }
            for (_, cell) in self.genome.cells.map_mut() {
                for input in cell.outputs.iter_mut() {
                    if let Some(inputs) = remove_queue.0.get(&input.id()) {
                        remove_tool(input, inputs);
                    }
                }
            }
            for hidden in self.genome.hidden.iter_mut() {
                if let Some(inputs) = remove_queue.0.get(&hidden.id()) {
                    remove_tool(hidden, inputs);
                }
            }
        }
    }
}

fn remove_tool<T: TakesInput>(node: &mut NeuronTopology<T>, values: &HashSet<Uuid>) {
    node.with_mut(|neuron| {
        neuron
            .inputs_mut()
            .retain(|input| input.id().is_some_and(|id| !values.contains(&id)));
    })
}

#[derive(Debug, Default)]
struct RemoveFrom(HashMap<Uuid, HashSet<Uuid>>);

fn dfs<T: TakesInput>(
    node: &NeuronTopology<T>,
    stack: &mut HashSet<Uuid>,
    visited: &mut HashSet<Uuid>,
) -> RemoveFrom {
    let node_id = node.id();
    visited.insert(node_id);

    let mut total_remove = RemoveFrom::default();
    let mut self_remove = HashSet::new();

    stack.insert(node_id);

    node.with_ref(|node| {
        for input in node.inputs() {
            match &input.input_type {
                NeuronInputType::Hidden(hidden) => {
                    if let Some(neuron_input) = hidden.upgrade() {
                        let input_id = neuron_input.id();
                        if !visited.contains(&input_id) {
                            let child_result = dfs(&neuron_input, stack, visited);
                            total_remove.0.extend(child_result.0);
                        } else if stack.contains(&input_id) {
                            self_remove.insert(input_id);
                        }
                    }
                }
                NeuronInputType::Input(_) => {}
            }
        }
    });

    if !self_remove.is_empty() {
        total_remove.0.insert(node_id, self_remove);
    }

    stack.remove(&node_id);
    total_remove
}
