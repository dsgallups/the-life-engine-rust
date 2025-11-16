mod topology;

pub use topology::*;

mod neuron_type;
pub use neuron_type::*;

#[cfg(test)]
use {
    pretty_assertions::assert_eq,
    rand::{Rng, SeedableRng, rngs::StdRng},
};

#[test]
fn test_input_neuron_creation() {
    let input = NeuronTopology::input();

    // Input neurons have unique IDs
    let id = input.id();
    assert_ne!(id, uuid::Uuid::nil(), "Input should have a valid ID");

    // Create another input and verify different ID
    let input2 = NeuronTopology::input();
    assert_ne!(
        input.id(),
        input2.id(),
        "Different inputs should have different IDs"
    );
}

#[test]
fn test_hidden_neuron_creation() {
    let hidden = NeuronTopology::hidden();

    // Hidden neurons start with no inputs
    hidden.with_ref(|neuron| {
        assert_eq!(
            neuron.inputs().len(),
            0,
            "Hidden should start with no inputs"
        );
        assert_eq!(neuron.bias(), 0.0, "Hidden should start with zero bias");
    });

    // Hidden neurons have unique IDs
    let hidden2 = NeuronTopology::hidden();
    assert_ne!(
        hidden.id(),
        hidden2.id(),
        "Different hidden neurons should have different IDs"
    );
}

#[test]
fn test_output_neuron_creation() {
    let output = NeuronTopology::output();

    // Output neurons start with no inputs
    output.with_ref(|neuron| {
        assert_eq!(
            neuron.inputs().len(),
            0,
            "Output should start with no inputs"
        );
        assert_eq!(neuron.bias(), 0.0, "Output should start with zero bias");
    });

    // Output neurons have unique IDs
    let output2 = NeuronTopology::output();
    assert_ne!(
        output.id(),
        output2.id(),
        "Different outputs should have different IDs"
    );
}

#[test]
fn test_add_input_to_hidden() {
    let hidden = NeuronTopology::hidden();
    let input1 = NeuronTopology::input();
    let input2 = NeuronTopology::input();

    // Add inputs
    hidden.add_input(&input1);
    hidden.add_input(&input2);

    // Verify inputs were added
    hidden.with_ref(|neuron| {
        assert_eq!(neuron.inputs().len(), 2, "Should have 2 inputs");

        // Each input should have default weight of 1.0
        for input in neuron.inputs() {
            assert_eq!(input.weight, 1.0, "Default weight should be 1.0");
            assert!(input.is_alive(), "Input should be alive");
        }
    });
}

#[test]
fn test_add_input_to_output() {
    let output = NeuronTopology::output();
    let hidden1 = NeuronTopology::hidden();
    let hidden2 = NeuronTopology::hidden();

    // Add inputs
    output.add_input(&hidden1);
    output.add_input(&hidden2);

    // Verify inputs were added
    output.with_ref(|neuron| {
        assert_eq!(neuron.inputs().len(), 2, "Should have 2 inputs");
    });
}

#[test]
fn test_hidden_can_take_input_neurons() {
    let hidden = NeuronTopology::hidden();
    let input = NeuronTopology::input();

    hidden.add_input(&input);

    hidden.with_ref(|neuron| {
        assert_eq!(neuron.inputs().len(), 1);
        assert!(matches!(
            neuron.inputs()[0].input_type,
            NeuronInputType::Input(_)
        ));
    });
}

#[test]
fn test_hidden_can_take_hidden_neurons() {
    let hidden1 = NeuronTopology::hidden();
    let hidden2 = NeuronTopology::hidden();

    hidden1.add_input(&hidden2);

    hidden1.with_ref(|neuron| {
        assert_eq!(neuron.inputs().len(), 1);
        assert!(matches!(
            neuron.inputs()[0].input_type,
            NeuronInputType::Hidden(_)
        ));
    });
}

#[test]
fn test_weak_reference_cleanup() {
    let output = NeuronTopology::output();
    let hidden = NeuronTopology::hidden();

    // Add hidden as input to output
    output.add_input(&hidden);

    // Verify connection exists
    output.with_ref(|neuron| {
        assert_eq!(neuron.inputs().len(), 1);
        assert!(neuron.inputs()[0].is_alive());
    });

    // Drop the hidden neuron
    drop(hidden);

    // Connection should now be dead
    output.with_ref(|neuron| {
        assert_eq!(neuron.inputs().len(), 1, "Dead connection still exists");
        assert!(!neuron.inputs()[0].is_alive(), "Connection should be dead");
    });
}

#[test]
fn test_neuron_input_id_retrieval() {
    let hidden = NeuronTopology::hidden();
    let input = NeuronTopology::input();

    let input_id = input.id();
    hidden.add_input(&input);

    hidden.with_ref(|neuron| {
        let connection_id = neuron.inputs()[0].id();
        assert_eq!(
            connection_id,
            Some(input_id),
            "Should retrieve correct input ID"
        );
    });
}

#[test]
fn test_neuron_input_dead_id() {
    let output = NeuronTopology::output();
    {
        let hidden = NeuronTopology::hidden();
        output.add_input(&hidden);
    } // hidden dropped here

    output.with_ref(|neuron| {
        let connection_id = neuron.inputs()[0].id();
        assert_eq!(
            connection_id, None,
            "Dead connection should return None for ID"
        );
    });
}

#[test]
fn test_bias_manipulation() {
    let hidden = NeuronTopology::hidden();

    // Set bias
    hidden.with_mut(|neuron| {
        *neuron.bias_mut() = 2.5;
    });

    // Verify bias was set
    hidden.with_ref(|neuron| {
        assert_eq!(neuron.bias(), 2.5, "Bias should be set to 2.5");
    });
}

#[test]
fn test_activation_function_setting() {
    let hidden = NeuronTopology::hidden();

    // Set different activation functions
    hidden.with_mut(|neuron| {
        neuron.set_activation(crate::ff_network::genome::activations::sigmoid);
    });

    // Verify activation function works
    hidden.with_ref(|neuron| {
        let activation = neuron.activation();
        // Test sigmoid behavior
        assert!(
            (activation(0.0) - 0.5).abs() < 0.001,
            "Sigmoid(0) should be 0.5"
        );
        assert!(activation(10.0) > 0.99, "Sigmoid(10) should be close to 1");
        assert!(
            activation(-10.0) < 0.01,
            "Sigmoid(-10) should be close to 0"
        );
    });

    // Change to relu
    hidden.with_mut(|neuron| {
        neuron.set_activation(crate::ff_network::genome::activations::relu);
    });

    hidden.with_ref(|neuron| {
        let activation = neuron.activation();
        assert_eq!(activation(-1.0), 0.0, "ReLU(-1) should be 0");
        assert_eq!(activation(5.0), 5.0, "ReLU(5) should be 5");
    });

    // Change to linear
    hidden.with_mut(|neuron| {
        neuron.set_activation(crate::ff_network::genome::activations::linear_activation);
    });

    hidden.with_ref(|neuron| {
        let activation = neuron.activation();
        assert_eq!(activation(-5.0), -5.0, "Linear(-5) should be -5");
        assert_eq!(activation(3.0), 3.0, "Linear(3) should be 3");
    });
}

#[test]
fn test_random_input_selection() {
    let mut rng = StdRng::seed_from_u64(42);
    let hidden = NeuronTopology::hidden();

    // Add multiple inputs
    let inputs: Vec<_> = (0..5).map(|_| NeuronTopology::input()).collect();
    for input in &inputs {
        hidden.add_input(input);
    }

    // Select random input and modify weight
    let modified = hidden.for_random_input(&mut rng, |input, rng| {
        input.weight += rng.random_range(-1.0..=1.0);
        true
    });

    assert_eq!(modified, Some(true), "Should have modified an input");

    // Verify at least one weight changed
    hidden.with_ref(|neuron| {
        let non_default_weights = neuron
            .inputs()
            .iter()
            .filter(|input| (input.weight - 1.0).abs() > 0.0001)
            .count();
        assert!(
            non_default_weights > 0,
            "At least one weight should have changed"
        );
    });
}

#[test]
fn test_random_input_on_empty() {
    let mut rng = StdRng::seed_from_u64(42);
    let hidden = NeuronTopology::hidden();

    // Try to select random input when there are none
    let result = hidden.for_random_input(&mut rng, |input, _rng| {
        input.weight = 5.0;
        "modified"
    });

    assert_eq!(result, None, "Should return None when no inputs exist");
}

#[test]
fn test_weight_mutation() {
    let mut rng = StdRng::seed_from_u64(42);
    let output = NeuronTopology::output();

    // Add inputs
    for _ in 0..3 {
        let hidden = NeuronTopology::hidden();
        output.add_input(&hidden);
    }

    // Mutate weights multiple times
    for _ in 0..10 {
        output.for_random_input(&mut rng, |input, rng| {
            input.weight += rng.random_range(-1.0..=1.0);
        });
    }

    // Check that weights have been modified
    output.with_ref(|neuron| {
        let modified_count = neuron
            .inputs()
            .iter()
            .filter(|input| (input.weight - 1.0).abs() > 0.0001)
            .count();
        assert!(modified_count > 0, "Some weights should be modified");
    });
}

#[test]
fn test_can_be_input_trait() {
    let input_neuron = NeuronTopology::input();
    let hidden_neuron = NeuronTopology::hidden();

    // Both Input and Hidden implement CanBeInput
    let input_type_from_input = input_neuron.to_input_type();
    let input_type_from_hidden = hidden_neuron.to_input_type();

    // Check they produce correct variants
    assert!(matches!(input_type_from_input, NeuronInputType::Input(_)));
    assert!(matches!(input_type_from_hidden, NeuronInputType::Hidden(_)));

    // Test equals method
    assert!(input_neuron.equals(&input_type_from_input));
    assert!(!input_neuron.equals(&input_type_from_hidden));
    assert!(hidden_neuron.equals(&input_type_from_hidden));
    assert!(!hidden_neuron.equals(&input_type_from_input));
}

#[test]
fn test_neuron_topology_clone() {
    use std::f32::consts::PI;
    let hidden1 = NeuronTopology::hidden();
    let hidden2 = hidden1.clone();

    // Cloned topology should share the same underlying neuron
    assert_eq!(hidden1.id(), hidden2.id(), "Clones should share same ID");

    // Modifying through one should affect the other
    hidden1.with_mut(|neuron| {
        *neuron.bias_mut() = PI;
    });

    hidden2.with_ref(|neuron| {
        assert_eq!(neuron.bias(), PI, "Changes should be visible through clone");
    });
}

#[test]
fn test_complex_network_construction() {
    // Build a small network to test interconnections
    let input1 = NeuronTopology::input();
    let input2 = NeuronTopology::input();
    let hidden1 = NeuronTopology::hidden();
    let hidden2 = NeuronTopology::hidden();
    let hidden3 = NeuronTopology::hidden();
    let output1 = NeuronTopology::output();
    let output2 = NeuronTopology::output();

    // Layer 1: Inputs to first hidden layer
    hidden1.add_input(&input1);
    hidden1.add_input(&input2);
    hidden2.add_input(&input1);
    hidden2.add_input(&input2);

    // Layer 2: Hidden to hidden
    hidden3.add_input(&hidden1);
    hidden3.add_input(&hidden2);

    // Layer 3: Hidden to outputs
    output1.add_input(&hidden1);
    output1.add_input(&hidden3);
    output2.add_input(&hidden2);
    output2.add_input(&hidden3);

    // Verify structure
    hidden1.with_ref(|neuron| {
        assert_eq!(neuron.inputs().len(), 2, "hidden1 should have 2 inputs");
    });

    hidden3.with_ref(|neuron| {
        assert_eq!(neuron.inputs().len(), 2, "hidden3 should have 2 inputs");
    });

    output1.with_ref(|neuron| {
        assert_eq!(neuron.inputs().len(), 2, "output1 should have 2 inputs");
    });

    output2.with_ref(|neuron| {
        assert_eq!(neuron.inputs().len(), 2, "output2 should have 2 inputs");
    });
}

#[test]
fn test_upgrade_weak_references() {
    let hidden = NeuronTopology::hidden();
    let input = NeuronTopology::input();

    hidden.add_input(&input);

    // Get the weak reference
    let weak_ref = hidden.with_ref(|neuron| match &neuron.inputs()[0].input_type {
        NeuronInputType::Input(weak) => weak.clone(),
        _ => panic!("Expected input type"),
    });

    // Should be able to upgrade while input is alive
    assert!(weak_ref.upgrade().is_some(), "Should upgrade successfully");

    // Drop the input
    drop(input);

    // Should not be able to upgrade after dropping
    assert!(
        weak_ref.upgrade().is_none(),
        "Should not upgrade after dropping"
    );
}

#[test]
fn test_mixed_input_types() {
    let output = NeuronTopology::output();
    let input1 = NeuronTopology::input();
    let input2 = NeuronTopology::input();
    let hidden1 = NeuronTopology::hidden();
    let hidden2 = NeuronTopology::hidden();

    // Add mixed types of inputs
    output.add_input(&input1);
    output.add_input(&hidden1);
    output.add_input(&input2);
    output.add_input(&hidden2);

    // Verify all were added
    output.with_ref(|neuron| {
        assert_eq!(neuron.inputs().len(), 4, "Should have 4 inputs");

        // Count each type
        let mut input_count = 0;
        let mut hidden_count = 0;

        for input in neuron.inputs() {
            match &input.input_type {
                NeuronInputType::Input(_) => input_count += 1,
                NeuronInputType::Hidden(_) => hidden_count += 1,
            }
        }

        assert_eq!(input_count, 2, "Should have 2 input neurons");
        assert_eq!(hidden_count, 2, "Should have 2 hidden neurons");
    });
}

#[test]
fn test_new_from_raw_parts() {
    let input1 = NeuronTopology::input();
    let input2 = NeuronTopology::input();

    let inputs = vec![
        NeuronInput {
            input_type: input1.to_input_type(),
            weight: 0.5,
        },
        NeuronInput {
            input_type: input2.to_input_type(),
            weight: -0.3,
        },
    ];

    let hidden =
        Hidden::new_from_raw_parts(inputs, 1.5, crate::ff_network::genome::activations::sigmoid);

    assert_eq!(hidden.inputs().len(), 2);
    assert_eq!(hidden.inputs()[0].weight, 0.5);
    assert_eq!(hidden.inputs()[1].weight, -0.3);
    assert_eq!(hidden.bias(), 1.5);
}

#[test]
fn test_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let hidden = NeuronTopology::hidden();
    let hidden_arc = Arc::new(hidden);

    let mut handles = vec![];

    // Spawn multiple threads that access the neuron
    for i in 0..5 {
        let hidden_clone = Arc::clone(&hidden_arc);
        let handle = thread::spawn(move || {
            // Each thread modifies the bias
            hidden_clone.with_mut(|neuron| {
                *neuron.bias_mut() += i as f32;
            });

            // And reads it back
            hidden_clone.with_ref(|neuron| {
                let _ = neuron.bias();
            });
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Final bias should be sum of all additions: 0 + 1 + 2 + 3 + 4 = 10
    hidden_arc.with_ref(|neuron| {
        assert_eq!(
            neuron.bias(),
            10.0,
            "Bias should be sum of all thread additions"
        );
    });
}
