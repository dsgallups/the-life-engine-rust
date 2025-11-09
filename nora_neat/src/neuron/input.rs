use std::sync::Weak;

use crate::prelude::*;
use rand::Rng;
use uuid::Uuid;

/// Represents a weighted input connection in a polynomial neural network.
///
/// Each `PolyInput` encapsulates:
/// - The source of the input (typically a neuron identifier)
/// - The connection weight
/// - The exponent applied to the input value
#[derive(Debug)]
pub struct NeuronInput<I = Topology> {
    id: Uuid,
    node: I,
    weight: f32,
}
impl<I: Clone> Clone for NeuronInput<I> {
    fn clone(&self) -> Self {
        Self {
            id: Uuid::new_v4(),
            node: self.node.clone(),
            weight: self.weight,
        }
    }
}

impl<I> NeuronInput<I> {
    /// Creates a new `PolyInput` with specified parameters.
    pub fn new(node: I, weight: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            node,
            weight,
        }
    }

    /// Creates a new `PolyInput` with random weight and exponent.
    pub fn new_rand(input: I, rng: &mut impl Rng) -> Self {
        Self::new(input, rng.random_range(-1.0..=1.0))
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Returns a reference to the input identifier.
    pub fn node(&self) -> &I {
        &self.node
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
    pub fn neuron(&self) -> Option<NeuronTopology> {
        Weak::upgrade(self.node().handle()).map(NeuronTopology::from_inner)
    }

    pub fn downgrade(input: &NeuronTopology, weight: f32) -> Self {
        Self::new(Topology::new(input), weight)
    }
}
