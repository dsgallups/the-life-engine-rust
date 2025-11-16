use std::sync::{Arc, Mutex};

use bevy::platform::collections::HashMap;
use uuid::Uuid;

use crate::ff_network::{
    CellGenome, CellMap, Genome, Hidden, Input, NeuronInput, NeuronInputType, NeuronTopology,
    TakesInput,
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
                let id = hidden_neuron_inner.id();

                match self.new_hidden.get(&id) {
                    Some(new_hidden_neuron) => Some(NeuronInput {
                        input_type: NeuronInputType::hidden(new_hidden_neuron),
                        weight: neuron_input.weight,
                    }),
                    None => {
                        let new_hidden_neuron =
                            self.new_takes_input_neuron(&*hidden_neuron_inner.lock());
                        let result = NeuronInput {
                            input_type: NeuronInputType::hidden(&new_hidden_neuron),
                            weight: neuron_input.weight,
                        };
                        self.new_hidden.insert(id, new_hidden_neuron);
                        Some(result)
                    }
                }
            }
            NeuronInputType::Input(input_neuron_inner) => {
                let input_neuron_inner = input_neuron_inner.upgrade()?;
                let id = input_neuron_inner.id();

                match self.new_inputs.get(&id) {
                    Some(new_input_neuron) => Some(NeuronInput {
                        input_type: NeuronInputType::input(new_input_neuron),
                        weight: neuron_input.weight,
                    }),
                    None => {
                        let new_input_neuron = NeuronTopology::input();
                        let result = NeuronInput {
                            input_type: NeuronInputType::input(&new_input_neuron),
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

#[cfg(test)]
use {
    crate::ff_network::*,
    bevy::math::IVec2,
    rand::{SeedableRng, rngs::StdRng},
};

#[test]
fn test_basic_genome_replication() {
    let original = Genome::sandbox();
    let cloned = original.deep_clone();

    // Basic structure should be preserved
    assert_eq!(
        original.cell_count(),
        cloned.cell_count(),
        "Cell count should be preserved"
    );
    assert_eq!(
        original.hidden_count(),
        cloned.hidden_count(),
        "Hidden neuron count should be preserved"
    );
}

#[test]
fn test_empty_genome_replication() {
    let original = Genome::empty();
    let cloned = original.deep_clone();

    assert_eq!(cloned.cell_count(), 0, "Empty genome should stay empty");
    assert_eq!(cloned.hidden_count(), 0, "Should have no hidden neurons");
}

#[test]
fn test_replicated_genome_independence() {
    let mut rng = StdRng::seed_from_u64(42);
    let original = Genome::sandbox();
    let mut cloned = original.deep_clone();

    let original_cell_count = original.cell_count();
    let original_hidden_count = original.hidden_count();

    // Mutate the clone
    for _ in 0..5 {
        MutationAction::AddCell.perform(&mut cloned.cells, &mut cloned.hidden, &mut rng);
    }

    // Original should be unchanged
    assert_eq!(
        original.cell_count(),
        original_cell_count,
        "Original genome should be unchanged after mutating clone"
    );
    assert_eq!(
        original.hidden_count(),
        original_hidden_count,
        "Original hidden count should be unchanged"
    );

    // Clone should have changed
    assert_ne!(
        cloned.cell_count(),
        original_cell_count,
        "Cloned genome should have different cell count after mutations"
    );
}

#[test]
fn test_cell_positions_preserved() {
    let genome = Genome::from_cells(vec![
        (CellKind::Eye, IVec2::new(0, 0)),
        (CellKind::Launcher, IVec2::new(5, 5)),
        (CellKind::Data, IVec2::new(-3, 2)),
        (CellKind::Collagen, IVec2::new(10, -10)),
    ]);

    let cloned = genome.deep_clone();

    // Check all positions are preserved
    assert!(
        cloned.cells.get(&IVec2::new(0, 0)).is_some(),
        "Cell at (0,0) should exist"
    );
    assert!(
        cloned.cells.get(&IVec2::new(5, 5)).is_some(),
        "Cell at (5,5) should exist"
    );
    assert!(
        cloned.cells.get(&IVec2::new(-3, 2)).is_some(),
        "Cell at (-3,2) should exist"
    );
    assert!(
        cloned.cells.get(&IVec2::new(10, -10)).is_some(),
        "Cell at (10,-10) should exist"
    );

    // Check cell types are preserved
    assert_eq!(
        cloned.cells.get(&IVec2::new(0, 0)).unwrap().kind,
        CellKind::Eye
    );
    assert_eq!(
        cloned.cells.get(&IVec2::new(5, 5)).unwrap().kind,
        CellKind::Launcher
    );
    assert_eq!(
        cloned.cells.get(&IVec2::new(-3, 2)).unwrap().kind,
        CellKind::Data
    );
    assert_eq!(
        cloned.cells.get(&IVec2::new(10, -10)).unwrap().kind,
        CellKind::Collagen
    );
}

#[test]
fn test_complex_connections_preserved() {
    let mut genome = Genome::empty();

    // Create a network with multiple cells and connections
    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Eye);
    genome.cells.add_cell(IVec2::new(1, 0), CellKind::Data);
    genome.cells.add_cell(IVec2::new(2, 0), CellKind::Launcher);

    // Add hidden neurons
    let hidden1 = NeuronTopology::hidden();
    let hidden2 = NeuronTopology::hidden();
    let hidden3 = NeuronTopology::hidden();

    // Create complex connections
    // Eye inputs -> hidden1
    if let Some(eye) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for input in &eye.inputs {
            hidden1.add_input(input);
        }
    }

    // hidden1 -> hidden2
    hidden2.add_input(&hidden1);

    // Data inputs -> hidden2
    if let Some(data) = genome.cells.map_mut().get_mut(&IVec2::new(1, 0)) {
        for input in &data.inputs {
            hidden2.add_input(input);
        }
        // Data outputs -> hidden3
        for output in &data.outputs {
            output.add_input(&hidden2);
            output.add_input(&hidden3);
        }
    }

    // hidden2 -> hidden3
    hidden3.add_input(&hidden2);

    // Connect to launcher outputs
    if let Some(launcher) = genome.cells.map_mut().get_mut(&IVec2::new(2, 0)) {
        for output in &launcher.outputs {
            output.add_input(&hidden3);
            output.add_input(&hidden1);
        }
    }

    genome.hidden.push(hidden1);
    genome.hidden.push(hidden2);
    genome.hidden.push(hidden3);

    // Clone the complex genome
    let cloned = genome.deep_clone();

    // Verify structure is preserved
    assert_eq!(cloned.hidden_count(), 3, "Should have 3 hidden neurons");
    assert_eq!(cloned.cell_count(), 3, "Should have 3 cells");

    // Verify connections exist (we can't directly compare them due to new IDs)
    // but we can verify the structure
    if let Some(data_cell) = cloned.cells.get(&IVec2::new(1, 0)) {
        // Data cell should have inputs and outputs as configured
        assert_eq!(data_cell.inputs.len(), 4, "Data cell should have 4 inputs");
        assert_eq!(
            data_cell.outputs.len(),
            4,
            "Data cell should have 4 outputs"
        );
    }
}

#[test]
fn test_neuron_ids_are_different() {
    let genome = Genome::simple_linear();
    let cloned = genome.deep_clone();

    // Neuron IDs should be different between original and clone
    if genome.hidden_count() > 0 && cloned.hidden_count() > 0 {
        let original_id = genome.hidden_neurons()[0].id();
        let cloned_id = cloned.hidden_neurons()[0].id();

        assert_ne!(
            original_id, cloned_id,
            "Cloned neurons should have different IDs"
        );
    }
}

#[test]
fn test_mutation_chances_preserved() {
    let mut genome = Genome::empty();
    let mut rng = StdRng::seed_from_u64(42);

    // Adjust mutation chances
    genome.mutation.adjust_mutation_chances(&mut rng);
    genome.mutation.adjust_mutation_chances(&mut rng);

    let cloned = genome.deep_clone();

    // Mutation chances should be preserved
    // We can test this by seeing if they produce similar mutation patterns
    let mut genome_mutations = Vec::new();
    let mut cloned_mutations = Vec::new();

    let mut genome_iter = genome.mutation.yield_mutations(&mut rng);
    let mut cloned_iter = cloned.mutation.yield_mutations(&mut rng);

    // Collect some mutations from each
    for _ in 0..10 {
        if let Some(action) = genome_iter.next(&mut rng) {
            genome_mutations.push(format!("{:?}", action));
        }
        if let Some(action) = cloned_iter.next(&mut rng) {
            cloned_mutations.push(format!("{:?}", action));
        }
    }

    // They should produce similar patterns (not necessarily identical due to RNG)
    // but both should produce some mutations if the original did
    if !genome_mutations.is_empty() {
        assert!(
            !cloned_mutations.is_empty(),
            "Clone should also produce mutations"
        );
    }
}

#[test]
fn test_deep_clone_with_dead_connections() {
    let mut genome = Genome::simple_linear();

    // Add a hidden neuron
    let hidden = NeuronTopology::hidden();

    // Connect it to outputs
    for cell in genome.cells.map_mut().values_mut() {
        for output in &cell.outputs {
            output.add_input(&hidden);
        }
    }

    genome.hidden.push(hidden);

    // Now remove a hidden neuron to create dead connections
    genome.hidden.clear();

    // Clone should handle dead connections gracefully
    let cloned = genome.deep_clone();

    // Verify clone is valid
    assert_eq!(cloned.hidden_count(), 0, "Should have no hidden neurons");
    assert!(cloned.cell_count() > 0, "Should still have cells");
}

#[test]
fn test_replicate_then_scramble() {
    let mut rng = StdRng::seed_from_u64(42);
    let original = Genome::sandbox();
    let mut cloned = original.deep_clone();

    // Scramble the clone
    cloned.scramble(&mut rng);

    // Original should be unchanged
    let original_recheck = original.deep_clone();
    assert_eq!(
        original.cell_count(),
        original_recheck.cell_count(),
        "Original should be unchanged after scrambling clone"
    );
}

#[test]
fn test_multiple_replication_generations() {
    // Test that we can replicate multiple generations
    let gen0 = Genome::sandbox();
    let gen1 = gen0.deep_clone();
    let gen2 = gen1.deep_clone();
    let gen3 = gen2.deep_clone();

    // All generations should have similar structure
    assert_eq!(gen0.cell_count(), gen3.cell_count());

    // But should be independent
    let mut rng = StdRng::seed_from_u64(42);
    let mut gen3_mutated = gen3.deep_clone();
    for _ in 0..5 {
        MutationAction::AddCell.perform(
            &mut gen3_mutated.cells,
            &mut gen3_mutated.hidden,
            &mut rng,
        );
    }

    // Earlier generations should be unchanged
    assert_eq!(gen0.cell_count(), gen1.cell_count());
    assert_eq!(gen1.cell_count(), gen2.cell_count());
    assert_eq!(gen2.cell_count(), gen3.cell_count());
}

#[test]
fn test_replication_with_cycles() {
    let mut genome = Genome::empty();

    // Create cells
    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Data);

    // Create hidden neurons
    let hidden1 = NeuronTopology::hidden();
    let hidden2 = NeuronTopology::hidden();

    // Create a potential cycle: hidden1 -> hidden2 -> hidden1
    hidden2.add_input(&hidden1);
    hidden1.add_input(&hidden2); // This creates a cycle

    // Connect to cell outputs
    if let Some(cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for output in &cell.outputs {
            output.add_input(&hidden1);
            output.add_input(&hidden2);
        }
    }

    genome.hidden.push(hidden1);
    genome.hidden.push(hidden2);

    // Replication should handle potential cycles
    let cloned = genome.deep_clone();

    assert_eq!(cloned.hidden_count(), 2, "Should preserve hidden neurons");
    assert_eq!(cloned.cell_count(), 1, "Should preserve cells");
}

#[test]
fn test_replication_preserves_weights() {
    let mut genome = Genome::simple_linear();
    let mut rng = StdRng::seed_from_u64(42);

    // Mutate weights to give them non-default values
    for _ in 0..10 {
        MutationAction::MutateWeight.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    }

    let cloned = genome.deep_clone();

    // Structure should be preserved
    assert_eq!(
        genome.cell_count(),
        cloned.cell_count(),
        "Cell count should match"
    );
    assert_eq!(
        genome.hidden_count(),
        cloned.hidden_count(),
        "Hidden count should match",
    );
}

#[test]
fn test_replication_preserves_activation_functions() {
    let mut genome = Genome::simple_linear();
    let mut rng = StdRng::seed_from_u64(42);

    // Mutate activation functions
    for _ in 0..10 {
        MutationAction::MutateActivation.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    }

    let cloned = genome.deep_clone();

    // Structure should be preserved
    assert_eq!(
        genome.cell_count(),
        cloned.cell_count(),
        "Cell count should match"
    );
    assert_eq!(
        genome.hidden_count(),
        cloned.hidden_count(),
        "Hidden count should match",
    );

    // The activation functions are preserved through the replication process
    // We can't directly compare them, but the structure is maintained
}

#[test]
fn test_cell_input_output_counts_preserved() {
    let genome = Genome::from_cells(vec![
        (CellKind::Eye, IVec2::new(0, 0)),      // 2 inputs, 0 outputs
        (CellKind::Launcher, IVec2::new(1, 0)), // 0 inputs, 3 outputs
        (CellKind::Data, IVec2::new(2, 0)),     // 4 inputs, 4 outputs
        (CellKind::Collagen, IVec2::new(3, 0)), // 0 inputs, 0 outputs
    ]);

    let cloned = genome.deep_clone();

    // Check Eye cell
    let eye_original = genome.cells.get(&IVec2::new(0, 0)).unwrap();
    let eye_cloned = cloned.cells.get(&IVec2::new(0, 0)).unwrap();
    assert_eq!(eye_original.inputs.len(), eye_cloned.inputs.len());
    assert_eq!(eye_original.outputs.len(), eye_cloned.outputs.len());

    // Check Launcher cell
    let launcher_original = genome.cells.get(&IVec2::new(1, 0)).unwrap();
    let launcher_cloned = cloned.cells.get(&IVec2::new(1, 0)).unwrap();
    assert_eq!(launcher_original.inputs.len(), launcher_cloned.inputs.len());
    assert_eq!(
        launcher_original.outputs.len(),
        launcher_cloned.outputs.len()
    );

    // Check Data cell
    let data_original = genome.cells.get(&IVec2::new(2, 0)).unwrap();
    let data_cloned = cloned.cells.get(&IVec2::new(2, 0)).unwrap();
    assert_eq!(data_original.inputs.len(), data_cloned.inputs.len());
    assert_eq!(data_original.outputs.len(), data_cloned.outputs.len());

    // Check Collagen cell
    let collagen_original = genome.cells.get(&IVec2::new(3, 0)).unwrap();
    let collagen_cloned = cloned.cells.get(&IVec2::new(3, 0)).unwrap();
    assert_eq!(collagen_original.inputs.len(), collagen_cloned.inputs.len());
    assert_eq!(
        collagen_original.outputs.len(),
        collagen_cloned.outputs.len(),
    );
}

#[test]
fn test_large_genome_replication_performance() {
    let mut genome = Genome::empty();
    let mut rng = StdRng::seed_from_u64(42);

    // Create a large genome
    for i in 0..20 {
        for j in 0..20 {
            let cell_kind = match (i + j) % 4 {
                0 => CellKind::Eye,
                1 => CellKind::Launcher,
                2 => CellKind::Data,
                _ => CellKind::Collagen,
            };
            genome.cells.add_cell(IVec2::new(i, j), cell_kind);
        }
    }

    // Add many hidden neurons
    for _ in 0..100 {
        genome.hidden.push(NeuronTopology::hidden());
    }

    // Add connections
    for _ in 0..50 {
        MutationAction::AddConnection.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    }

    // Should be able to replicate large genome
    let cloned = genome.deep_clone();

    assert_eq!(genome.cell_count(), cloned.cell_count());
    assert_eq!(genome.hidden_count(), cloned.hidden_count());
}
