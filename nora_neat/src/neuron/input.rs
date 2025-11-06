use std::sync::{Arc, RwLock, Weak};

use crate::prelude::*;
use rand::Rng;

/// Represents a weighted input connection in a polynomial neural network.
///
/// Each `PolyInput` encapsulates:
/// - The source of the input (typically a neuron identifier)
/// - The connection weight
/// - The exponent applied to the input value
#[derive(Clone, Debug)]
pub struct NeuronInput<I> {
    input: I,
    weight: f32,
}

impl<I> NeuronInput<I> {
    /// Creates a new `PolyInput` with specified parameters.
    pub fn new(input: I, weight: f32) -> Self {
        Self { input, weight }
    }

    /// Creates a new `PolyInput` with random weight and exponent.
    pub fn new_rand(input: I, rng: &mut impl Rng) -> Self {
        Self {
            input,
            weight: rng.random_range(-1.0..=1.0),
        }
    }

    /// Returns a reference to the input identifier.
    pub fn input(&self) -> &I {
        &self.input
    }

    /// Returns the connection weight.
    pub fn weight(&self) -> f32 {
        self.weight
    }

    /// Adjusts the connection weight by adding the specified delta.
    pub fn adjust_weight(&mut self, by: f32) {
        self.weight += by;
    }
}

impl NeuronInput<Topology> {
    pub fn neuron(&self) -> Option<Arc<RwLock<NeuronTopology>>> {
        Weak::upgrade(self.input().handle())
    }

    pub fn downgrade(input: &Arc<RwLock<NeuronTopology>>, weight: f32) -> Self {
        Self::new(Topology::new(input), weight)
    }
}
