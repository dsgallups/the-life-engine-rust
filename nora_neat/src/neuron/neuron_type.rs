use std::{
    fmt,
    sync::{Arc, RwLock, Weak},
};

use rand::Rng;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeuronType {
    Input,
    Props(PropsType),
}

impl fmt::Display for NeuronType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input => write!(f, "Input"),
            Self::Props(PropsType::Hidden) => write!(f, "Hidden"),
            Self::Props(PropsType::Output) => write!(f, "Output"),
        }
    }
}

impl NeuronType {
    pub fn input() -> Self {
        Self::Input
    }
    pub fn hidden() -> Self {
        Self::Props(PropsType::Hidden)
    }
    pub fn output() -> Self {
        Self::Props(PropsType::Output)
    }
}

impl From<PropsType> for NeuronType {
    fn from(value: PropsType) -> Self {
        Self::Props(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropsType {
    Hidden,
    Output,
}

#[derive(Clone, Debug)]
pub struct NeuronProps<I> {
    pub(crate) props_type: PropsType,
    pub(crate) inputs: Vec<NeuronInput<I>>,
    bias: f32,
    activation: fn(f32) -> f32,
}

impl<I> NeuronProps<I> {
    pub fn new(
        props_type: PropsType,
        inputs: Vec<NeuronInput<I>>,
        bias: f32,
        activation: fn(f32) -> f32,
    ) -> Self {
        Self {
            props_type,
            inputs,
            bias,
            activation,
        }
    }
    pub fn hidden(inputs: Vec<NeuronInput<I>>, rng: &mut impl Rng) -> Self {
        Self::new(
            PropsType::Hidden,
            inputs,
            rng.random(),
            Activation::rand(rng),
        )
    }
    pub fn output(inputs: Vec<NeuronInput<I>>, rng: &mut impl Rng) -> Self {
        Self::new(
            PropsType::Output,
            inputs,
            rng.random(),
            Activation::rand(rng),
        )
    }

    pub fn num_inputs(&self) -> usize {
        self.inputs.len()
    }

    pub fn inputs(&self) -> &[NeuronInput<I>] {
        self.inputs.as_slice()
    }

    pub fn props_type(&self) -> PropsType {
        self.props_type
    }
}

#[derive(Debug, Clone)]
pub struct Topology(Weak<RwLock<NeuronTopology>>);

impl Topology {
    pub fn new(inner: &Arc<RwLock<NeuronTopology>>) -> Self {
        Self(Arc::downgrade(inner))
    }
    pub fn handle(&self) -> &Weak<RwLock<NeuronTopology>> {
        &self.0
    }
}
//pub type PolyNeuronPropsTopology = PolyProps<Weak<RwLock<NeuronTopology>>>;

impl NeuronProps<Topology> {
    pub fn set_inputs(&mut self, new_inputs: Vec<NeuronInput<Topology>>) {
        self.inputs = new_inputs;
    }

    /// resets the inputs and copies activation + bias
    pub fn deep_clone(&self) -> Self {
        Self {
            props_type: self.props_type,
            inputs: Vec::with_capacity(self.inputs.len()),
            activation: self.activation,
            bias: self.bias,
        }
    }

    pub fn add_input(&mut self, input: NeuronInput<Topology>) {
        self.inputs.push(input);
    }

    // Clears out all inputs whose reference is dropped or match on the provided ids
    pub fn trim_inputs(&mut self, indices: &[usize]) {
        let mut sorted_indices = indices.to_vec();
        sorted_indices.sort_unstable_by(|a, b| b.cmp(a));

        for index in sorted_indices {
            self.inputs.remove(index);
        }
    }

    /// Returnes the removed input, if it has inputs.
    pub fn remove_random_input(&mut self, rng: &mut impl Rng) -> Option<NeuronInput<Topology>> {
        if self.inputs.is_empty() {
            return None;
        }
        let removed = self
            .inputs
            .swap_remove(rng.random_range(0..self.inputs.len()));
        Some(removed)
    }

    pub fn get_random_input_mut(
        &mut self,
        rng: &mut impl Rng,
    ) -> Option<&mut NeuronInput<Topology>> {
        if self.inputs.is_empty() {
            return None;
        }
        let len = self.inputs.len();
        self.inputs.get_mut(rng.random_range(0..len))
    }
}
