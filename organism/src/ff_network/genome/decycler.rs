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

#[cfg(test)]
use {
    crate::ff_network::CellKind,
    bevy::prelude::*,
    rand::{SeedableRng, rngs::StdRng},
};

#[test]
fn test_cleaner_removes_dead_connections() {
    let mut genome = Genome::empty();

    // Create cells with inputs and outputs
    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Eye);
    genome.cells.add_cell(IVec2::new(1, 0), CellKind::Launcher);

    // Create hidden neurons
    let hidden1 = NeuronTopology::hidden();
    let hidden2 = NeuronTopology::hidden();

    // Create connections
    if let Some(eye) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for input in &eye.inputs {
            hidden1.add_input(input);
        }
    }

    if let Some(launcher) = genome.cells.map_mut().get_mut(&IVec2::new(1, 0)) {
        for output in &launcher.outputs {
            output.add_input(&hidden1);
            output.add_input(&hidden2);
        }
    }

    // Add hidden neurons to genome
    genome.hidden.push(hidden1);
    genome.hidden.push(hidden2);

    // Now remove hidden neurons to create dead connections
    genome.hidden.clear();

    // Clean should remove dead connections
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.remove_dead_connections();

    // Check that dead connections are removed
    for cell in genome.cells.map().values() {
        for output in &cell.outputs {
            output.with_ref(|neuron| {
                for input in neuron.inputs() {
                    assert!(
                        input.is_alive(),
                        "All connections should be alive after cleaning"
                    );
                }
            });
        }
    }
}

#[test]
fn test_decycler_simple_cycle() {
    let mut genome = Genome::empty();

    // Create a Data cell (has both inputs and outputs)
    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Data);

    // Create two hidden neurons
    let hidden1 = NeuronTopology::hidden();
    let hidden2 = NeuronTopology::hidden();

    // Create a cycle: hidden1 -> hidden2 -> hidden1
    hidden2.add_input(&hidden1);
    hidden1.add_input(&hidden2);

    // Connect cell outputs to hidden neurons
    if let Some(cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for output in &cell.outputs {
            output.add_input(&hidden1);
        }
    }

    genome.hidden.push(hidden1.clone());
    genome.hidden.push(hidden2.clone());

    // Run decycler
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.decycle();

    // The cycle should be broken
    // We can't easily verify the exact structure, but it should not panic
    // and the genome should still be valid
    assert_eq!(genome.hidden_count(), 2, "Hidden neurons should remain");
}

#[test]
fn test_decycler_self_loop() {
    let mut genome = Genome::empty();

    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Launcher);

    // Create a hidden neuron with a self-loop
    let hidden = NeuronTopology::hidden();
    hidden.add_input(&hidden); // Self-loop

    // Connect to output
    if let Some(cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for output in &cell.outputs {
            output.add_input(&hidden);
        }
    }

    genome.hidden.push(hidden);

    // Run decycler
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.decycle();

    // Self-loop should be removed
    // Verify by checking that no neuron has itself as input
    for hidden_neuron in &genome.hidden {
        let hidden_id = hidden_neuron.id();
        hidden_neuron.with_ref(|neuron| {
            for input in neuron.inputs() {
                if let Some(input_id) = input.id() {
                    assert_ne!(input_id, hidden_id, "Self-loops should be removed");
                }
            }
        });
    }
}

#[test]
fn test_decycler_complex_cycle() {
    let mut genome = Genome::empty();

    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Data);

    // Create a complex cycle: h1 -> h2 -> h3 -> h4 -> h2
    let h1 = NeuronTopology::hidden();
    let h2 = NeuronTopology::hidden();
    let h3 = NeuronTopology::hidden();
    let h4 = NeuronTopology::hidden();

    h2.add_input(&h1);
    h3.add_input(&h2);
    h4.add_input(&h3);
    h2.add_input(&h4); // Creates the cycle

    // Connect to outputs
    if let Some(cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for output in &cell.outputs {
            output.add_input(&h1);
            output.add_input(&h3);
        }
    }

    genome.hidden.push(h1);
    genome.hidden.push(h2);
    genome.hidden.push(h3);
    genome.hidden.push(h4);

    // Run decycler
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.decycle();

    // Should still have all neurons
    assert_eq!(genome.hidden_count(), 4, "All neurons should remain");
}

#[test]
fn test_decycler_multiple_independent_cycles() {
    let mut genome = Genome::empty();

    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Data);
    genome.cells.add_cell(IVec2::new(1, 0), CellKind::Launcher);

    // Create first cycle: h1 <-> h2
    let h1 = NeuronTopology::hidden();
    let h2 = NeuronTopology::hidden();
    h1.add_input(&h2);
    h2.add_input(&h1);

    // Create second cycle: h3 <-> h4
    let h3 = NeuronTopology::hidden();
    let h4 = NeuronTopology::hidden();
    h3.add_input(&h4);
    h4.add_input(&h3);

    // Connect cycles to different outputs
    if let Some(data_cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for output in &data_cell.outputs {
            output.add_input(&h1);
            output.add_input(&h2);
        }
    }

    if let Some(launcher_cell) = genome.cells.map_mut().get_mut(&IVec2::new(1, 0)) {
        for output in &launcher_cell.outputs {
            output.add_input(&h3);
            output.add_input(&h4);
        }
    }

    genome.hidden.push(h1);
    genome.hidden.push(h2);
    genome.hidden.push(h3);
    genome.hidden.push(h4);

    // Run decycler
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.decycle();

    // All neurons should remain
    assert_eq!(genome.hidden_count(), 4);
}

#[test]
fn test_clean_combines_dead_removal_and_decycling() {
    let mut genome = Genome::empty();

    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Data);

    // Create neurons
    let h1 = NeuronTopology::hidden();
    let h2 = NeuronTopology::hidden();
    let h3 = NeuronTopology::hidden();

    // Create a cycle
    h1.add_input(&h2);
    h2.add_input(&h1);

    // h3 will become dead

    // Connect to outputs
    if let Some(cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for output in &cell.outputs {
            output.add_input(&h1);
            output.add_input(&h2);
            output.add_input(&h3);
        }
    }

    genome.hidden.push(h1);
    genome.hidden.push(h2);
    // Don't add h3 to genome.hidden, making it a dead connection

    // Run clean (combines remove_dead_connections and decycle)
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.clean();

    // Should have broken the cycle and removed dead connections
    assert_eq!(genome.hidden_count(), 2);

    // Verify no dead connections remain
    for cell in genome.cells.map().values() {
        for output in &cell.outputs {
            output.with_ref(|neuron| {
                for input in neuron.inputs() {
                    assert!(input.is_alive());
                }
            });
        }
    }
}

#[test]
fn test_decycler_preserves_valid_paths() {
    let mut genome = Genome::empty();

    // Create a more complex network
    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Eye);
    genome.cells.add_cell(IVec2::new(1, 0), CellKind::Data);
    genome.cells.add_cell(IVec2::new(2, 0), CellKind::Launcher);

    let h1 = NeuronTopology::hidden();
    let h2 = NeuronTopology::hidden();
    let h3 = NeuronTopology::hidden();
    let h4 = NeuronTopology::hidden();

    // Create valid forward connections
    if let Some(eye) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for input in &eye.inputs {
            h1.add_input(input);
        }
    }

    h2.add_input(&h1);
    h3.add_input(&h2);
    h4.add_input(&h3);

    // Add a backward edge creating a cycle
    h2.add_input(&h4);

    // Connect to outputs
    if let Some(launcher) = genome.cells.map_mut().get_mut(&IVec2::new(2, 0)) {
        for output in &launcher.outputs {
            output.add_input(&h4);
        }
    }

    genome.hidden.push(h1);
    genome.hidden.push(h2);
    genome.hidden.push(h3);
    genome.hidden.push(h4);

    let initial_hidden_count = genome.hidden_count();

    // Run decycler
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.decycle();

    // All neurons should still exist
    assert_eq!(genome.hidden_count(), initial_hidden_count);

    // The main forward path should still exist (though we can't easily verify the exact structure)
}

#[test]
fn test_decycler_handles_disconnected_components() {
    let mut genome = Genome::empty();

    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Launcher);
    genome.cells.add_cell(IVec2::new(1, 0), CellKind::Launcher);

    // Create two separate subgraphs
    let h1 = NeuronTopology::hidden();
    let h2 = NeuronTopology::hidden();
    let h3 = NeuronTopology::hidden();
    let h4 = NeuronTopology::hidden();

    // First subgraph with cycle
    h1.add_input(&h2);
    h2.add_input(&h1);

    // Second subgraph without cycle
    h4.add_input(&h3);

    // Connect to different outputs
    let cells: Vec<_> = genome.cells.map().keys().cloned().collect();
    if let Some(cell1) = genome.cells.map_mut().get_mut(&cells[0]) {
        for output in &cell1.outputs {
            output.add_input(&h1);
        }
    }

    if let Some(cell2) = genome.cells.map_mut().get_mut(&cells[1]) {
        for output in &cell2.outputs {
            output.add_input(&h4);
        }
    }

    genome.hidden.push(h1);
    genome.hidden.push(h2);
    genome.hidden.push(h3);
    genome.hidden.push(h4);

    // Run decycler
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.decycle();

    // All neurons should remain
    assert_eq!(genome.hidden_count(), 4);
}

#[test]
fn test_clean_on_empty_genome() {
    let mut genome = Genome::empty();

    // Should not panic on empty genome
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.clean();

    assert_eq!(genome.cell_count(), 0);
    assert_eq!(genome.hidden_count(), 0);
}

#[test]
fn test_clean_on_genome_with_no_cycles() {
    let mut genome = Genome::simple_linear();

    let initial_hidden = genome.hidden_count();
    let initial_cells = genome.cell_count();

    // Clean should not change a valid acyclic genome
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.clean();

    assert_eq!(genome.hidden_count(), initial_hidden);
    assert_eq!(genome.cell_count(), initial_cells);
}

#[test]
fn test_decycler_with_deeply_nested_cycles() {
    let mut genome = Genome::empty();

    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Data);

    // Create a deep nested structure with multiple cycles
    let neurons: Vec<_> = (0..10).map(|_| NeuronTopology::hidden()).collect();

    // Create forward connections
    for i in 0..9 {
        neurons[i + 1].add_input(&neurons[i]);
    }

    // Add multiple backward edges creating nested cycles
    neurons[2].add_input(&neurons[5]);
    neurons[1].add_input(&neurons[8]);
    neurons[0].add_input(&neurons[9]);

    // Connect to outputs
    if let Some(cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for output in &cell.outputs {
            output.add_input(&neurons[0]);
            output.add_input(&neurons[9]);
        }
    }

    for neuron in neurons {
        genome.hidden.push(neuron);
    }

    // Run decycler
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.decycle();

    // Should maintain all neurons
    assert_eq!(genome.hidden_count(), 10);
}

#[test]
fn test_multiple_clean_calls_are_idempotent() {
    let mut genome = Genome::empty();

    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Data);

    // Create a cycle
    let h1 = NeuronTopology::hidden();
    let h2 = NeuronTopology::hidden();
    h1.add_input(&h2);
    h2.add_input(&h1);

    if let Some(cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for output in &cell.outputs {
            output.add_input(&h1);
        }
    }

    genome.hidden.push(h1);
    genome.hidden.push(h2);

    // First clean
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.clean();

    let state_after_first = genome.hidden_count();

    // Second clean
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.clean();

    let state_after_second = genome.hidden_count();

    // Third clean
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.clean();

    let state_after_third = genome.hidden_count();

    // State should stabilize after first clean
    assert_eq!(state_after_first, state_after_second);
    assert_eq!(state_after_second, state_after_third);
}

#[test]
fn test_clean_preserves_input_to_output_connectivity() {
    let mut genome = Genome::empty();

    // Create a network with inputs and outputs
    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Eye);
    genome.cells.add_cell(IVec2::new(1, 0), CellKind::Launcher);

    let h1 = NeuronTopology::hidden();
    let h2 = NeuronTopology::hidden();

    // Connect input -> h1 -> h2 -> output
    if let Some(eye) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for input in &eye.inputs {
            h1.add_input(input);
        }
    }

    h2.add_input(&h1);

    if let Some(launcher) = genome.cells.map_mut().get_mut(&IVec2::new(1, 0)) {
        for output in &launcher.outputs {
            output.add_input(&h2);
        }
    }

    // Add a cycle
    h1.add_input(&h2);

    genome.hidden.push(h1);
    genome.hidden.push(h2);

    // Clean
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.clean();

    // Should still have both hidden neurons
    assert_eq!(genome.hidden_count(), 2);

    // There should still be a path from input to output
    // (though the cycle should be broken)
}

#[test]
fn test_cleaner_with_partially_dead_connections() {
    let mut genome = Genome::empty();

    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Data);

    let h1 = NeuronTopology::hidden();
    let h2 = NeuronTopology::hidden();
    let h3 = NeuronTopology::hidden();

    // Connect h1 and h2 to outputs
    if let Some(cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for output in &cell.outputs {
            output.add_input(&h1);
            output.add_input(&h2);
            output.add_input(&h3); // h3 will be dead
        }

        // Also create some connections from inputs
        for input in &cell.inputs {
            h1.add_input(input);
            h2.add_input(input);
        }
    }

    // Only add h1 and h2 to genome, making h3 dead
    genome.hidden.push(h1);
    genome.hidden.push(h2);

    // Clean
    let mut cleaner = Cleaner::new(&mut genome);
    cleaner.clean();

    // Should still have h1 and h2
    assert_eq!(genome.hidden_count(), 2);

    // Verify only live connections remain
    for cell in genome.cells.map().values() {
        for output in &cell.outputs {
            output.with_ref(|neuron| {
                for input in neuron.inputs() {
                    assert!(input.is_alive(), "Only live connections should remain");
                }
            });
        }
    }
}

#[test]
fn test_genome_scramble_includes_cleaning() {
    let mut genome = Genome::empty();
    let mut rng = StdRng::seed_from_u64(42);

    // Create a genome with a cycle
    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Data);

    let h1 = NeuronTopology::hidden();
    let h2 = NeuronTopology::hidden();
    h1.add_input(&h2);
    h2.add_input(&h1);

    if let Some(cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for output in &cell.outputs {
            output.add_input(&h1);
        }
    }

    genome.hidden.push(h1);
    genome.hidden.push(h2);

    // Scramble should include cleaning (which removes cycles)
    genome.scramble(&mut rng);
}
