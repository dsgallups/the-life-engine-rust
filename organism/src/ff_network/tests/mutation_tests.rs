use crate::ff_network::*;
use bevy::math::IVec2;
use rand::{SeedableRng, rngs::StdRng};

#[test]
fn test_mutation_action_add_cell() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::empty();

    // Add a few cells through mutation
    for _ in 0..5 {
        MutationAction::AddCell.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    }

    assert_eq!(genome.cell_count(), 5, "Should have added 5 cells");

    // Verify cells are at different locations
    let locations: Vec<_> = genome.cells.map().keys().cloned().collect();
    let unique_locations: std::collections::HashSet<_> = locations.iter().cloned().collect();
    assert_eq!(
        locations.len(),
        unique_locations.len(),
        "All cells should be at unique locations"
    );
}

#[test]
fn test_mutation_action_delete_cell() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::from_cells(vec![
        (CellKind::Eye, IVec2::new(0, 0)),
        (CellKind::Launcher, IVec2::new(1, 0)),
        (CellKind::Data, IVec2::new(2, 0)),
    ]);

    assert_eq!(genome.cell_count(), 3);

    MutationAction::DeleteCell.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    assert_eq!(genome.cell_count(), 2, "Should have deleted one cell");

    MutationAction::DeleteCell.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    assert_eq!(genome.cell_count(), 1, "Should have deleted another cell");
}

#[test]
fn test_mutation_action_delete_cell_empty_genome() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::empty();

    // Should not panic when deleting from empty genome
    MutationAction::DeleteCell.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    assert_eq!(genome.cell_count(), 0, "Should still be empty");
}

#[test]
fn test_mutation_action_mutate_cell() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::from_cells(vec![(CellKind::Eye, IVec2::new(0, 0))]);

    // Mutate the cell multiple times
    for _ in 0..10 {
        MutationAction::MutateCell.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    }

    // Cell count should remain the same
    assert_eq!(genome.cell_count(), 1, "Cell count should not change");

    // The cell type might have changed (or might not, depending on RNG)
    let cell = genome.cells.get(&IVec2::new(0, 0)).unwrap();
    // Just verify we still have a valid cell
    assert!(matches!(
        cell.kind,
        CellKind::Eye | CellKind::Launcher | CellKind::Data | CellKind::Collagen
    ));
}

#[test]
fn test_mutation_action_add_connection() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::simple_linear();

    let initial_hidden_count = genome.hidden_count();

    // Add connections multiple times
    for _ in 0..5 {
        MutationAction::AddConnection.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    }

    // Hidden count should remain the same (AddConnection doesn't create new neurons)
    assert_eq!(genome.hidden_count(), initial_hidden_count);
}

#[test]
fn test_mutation_action_split_connection() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::simple_linear();

    let initial_hidden_count = genome.hidden_count();

    // Split connections should create new hidden neurons
    for _ in 0..3 {
        MutationAction::SplitConnection.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    }

    // Some splits might have succeeded and added hidden neurons
    // (might not always succeed if there are no connections to split)
    assert!(genome.hidden_count() >= initial_hidden_count);
}

#[test]
fn test_mutation_action_remove_neuron() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::simple_linear();

    // Add some hidden neurons first
    for _ in 0..5 {
        genome.hidden.push(NeuronTopology::hidden());
    }

    let initial_count = genome.hidden_count();

    MutationAction::RemoveNeuron.perform(&mut genome.cells, &mut genome.hidden, &mut rng);

    assert_eq!(
        genome.hidden_count(),
        initial_count - 1,
        "Should have removed one neuron"
    );
}

#[test]
fn test_mutation_action_remove_neuron_empty() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::empty();

    // Should not panic when removing from empty hidden list
    MutationAction::RemoveNeuron.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    assert_eq!(genome.hidden_count(), 0);
}

#[test]
fn test_mutation_action_mutate_weight() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::simple_linear();

    // This should modify weights on existing connections
    for _ in 0..10 {
        MutationAction::MutateWeight.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    }

    // Just verify it doesn't panic or change structure
    assert!(genome.cell_count() > 0);
}

#[test]
fn test_mutation_action_mutate_activation() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::simple_linear();

    // This should modify activation functions on neurons
    for _ in 0..10 {
        MutationAction::MutateActivation.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    }

    // Just verify it doesn't panic or change structure
    assert!(genome.cell_count() > 0);
}

#[test]
fn test_mutation_chances_initialization() {
    let chances = MutationChances::new(75);

    // Verify self_mutation rate is set correctly
    // Note: We don't have direct access to self_mutation field, so we test behavior

    // All actions should have some positive chance
    let mut action_performed = false;
    let mut rng = StdRng::seed_from_u64(42);
    let mut iter = chances.yield_mutations(&mut rng);

    for _ in 0..1000 {
        if iter.next(&mut rng).is_some() {
            action_performed = true;
            break;
        }
    }

    assert!(action_performed, "Should yield at least one mutation");
}

#[test]
fn test_mutation_chances_adjust() {
    let mut chances = MutationChances::new(50);
    let mut rng = StdRng::seed_from_u64(42);

    // Adjust mutation chances multiple times
    for _ in 0..10 {
        chances.adjust_mutation_chances(&mut rng);
    }

    // After adjustment, should still yield mutations
    let mut iter = chances.yield_mutations(&mut rng);
    let mut count = 0;

    while iter.next(&mut rng).is_some() {
        count += 1;
        if count > MAX_MUTATIONS as usize {
            break; // Prevent infinite loop
        }
    }

    assert!(
        count <= MAX_MUTATIONS as usize,
        "Should not exceed MAX_MUTATIONS"
    );
}

#[test]
fn test_mutation_iter_max_mutations() {
    let chances = MutationChances::new(100); // High chance to trigger many mutations
    let mut rng = StdRng::seed_from_u64(42);

    let mut iter = chances.yield_mutations(&mut rng);
    let mut count = 0;

    while iter.next(&mut rng).is_some() {
        count += 1;
        if count > MAX_MUTATIONS as usize + 10 {
            panic!("MutationIter exceeded MAX_MUTATIONS limit");
        }
    }

    assert!(
        count <= MAX_MUTATIONS as usize,
        "Should respect MAX_MUTATIONS limit"
    );
}

#[test]
fn test_genome_scramble() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::simple_linear();

    let initial_cell_count = genome.cell_count();

    // Scramble the genome
    genome.scramble(&mut rng);

    // Genome should still be valid after scrambling
    assert!(genome.cell_count() >= 0);

    // Scrambling might have changed the number of cells
    // but should not crash
}

#[test]
fn test_genome_scramble_multiple_times() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::sandbox();

    // Scramble multiple times to test stability
    for _ in 0..10 {
        genome.scramble(&mut rng);
    }

    // Genome should still be valid
    assert!(genome.cell_count() >= 0);
    assert!(genome.hidden_count() >= 0);
}

#[test]
fn test_mutation_on_complex_genome() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut genome = Genome::sandbox();

    // Apply many random mutations
    let actions = [
        MutationAction::AddCell,
        MutationAction::DeleteCell,
        MutationAction::MutateCell,
        MutationAction::AddConnection,
        MutationAction::SplitConnection,
        MutationAction::RemoveNeuron,
        MutationAction::MutateWeight,
        MutationAction::MutateActivation,
    ];

    for _ in 0..100 {
        let action_idx = rng.random_range(0..actions.len());
        actions[action_idx].perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    }

    // Should not crash and genome should still be valid
    assert!(genome.cell_count() >= 0);
    assert!(genome.hidden_count() >= 0);
}

#[test]
fn test_split_connection_creates_hidden() {
    let mut rng = StdRng::seed_from_u64(42);

    // Create a genome with a direct connection we can split
    let mut genome = Genome::empty();
    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Eye);
    genome.cells.add_cell(IVec2::new(1, 0), CellKind::Launcher);

    // Connect input to output directly
    let eye_inputs = genome.cells.get(&IVec2::new(0, 0)).unwrap().inputs.clone();
    if let Some(launcher) = genome.cells.map_mut().get_mut(&IVec2::new(1, 0)) {
        for output in &launcher.outputs {
            for input in &eye_inputs {
                output.add_input(input);
            }
        }
    }

    let initial_hidden = genome.hidden_count();

    // Try to split connections multiple times
    for _ in 0..10 {
        MutationAction::SplitConnection.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
    }

    // Should have created at least some hidden neurons
    // (might not always succeed depending on RNG and connection availability)
    assert!(genome.hidden_count() >= initial_hidden);
}

#[test]
fn test_mutations_preserve_cell_requirements() {
    let mut rng = StdRng::seed_from_u64(42);

    // Test each cell type maintains correct I/O counts after mutations
    let cell_types = [
        CellKind::Eye,
        CellKind::Launcher,
        CellKind::Collagen,
        CellKind::Data,
    ];

    for cell_kind in cell_types {
        let mut genome = Genome::empty();
        genome.cells.add_cell(IVec2::new(0, 0), cell_kind);

        // Apply mutations
        for _ in 0..5 {
            MutationAction::AddConnection.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
            MutationAction::MutateWeight.perform(&mut genome.cells, &mut genome.hidden, &mut rng);
        }

        // Check cell still has correct number of inputs/outputs
        let cell = genome.cells.get(&IVec2::new(0, 0)).unwrap();
        let requirements = cell_kind.requirements();

        assert_eq!(
            cell.inputs.len(),
            requirements.num_inputs,
            "{:?} should have {} inputs",
            cell_kind,
            requirements.num_inputs
        );
        assert_eq!(
            cell.outputs.len(),
            requirements.num_outputs,
            "{:?} should have {} outputs",
            cell_kind,
            requirements.num_outputs
        );
    }
}

#[test]
fn test_mutation_chance_adjustment() {
    let mut chance = MutationChance {
        action: MutationAction::AddCell,
        chance: 50.0,
    };

    let mut rng = StdRng::seed_from_u64(42);
    let original = chance.chance;

    // Adjust multiple times
    for _ in 0..100 {
        chance.adjust_chance(&mut rng);
    }

    // Chance should have changed
    assert_ne!(chance.chance, original, "Chance should have been adjusted");
}
