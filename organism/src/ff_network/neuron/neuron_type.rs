use bevy::ecs::system::In;
use rand::{
    Rng,
    seq::{IndexedMutRandom, IndexedRandom},
};
use uuid::Uuid;

use crate::ff_network::NeuronTopology;

use std::sync::{Arc, Mutex, Weak};

pub trait TopologyNeuron {
    fn id(&self) -> Uuid;
}

pub trait TakesInput: TopologyNeuron {
    fn new_from_raw_parts(inputs: Vec<NeuronInput>, bias: f32, activation: fn(f32) -> f32) -> Self;
    fn add_input(&mut self, input: impl Into<NeuronInputType>);
    fn inputs(&self) -> &[NeuronInput];
    fn bias(&self) -> f32;
    fn activation(&self) -> fn(f32) -> f32;
    fn mutate_random_weight(&mut self, rng: &mut impl Rng);
}

#[derive(Clone)]
pub struct Input {
    id: Uuid,
}
impl Default for Input {
    fn default() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

impl TopologyNeuron for Input {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Clone)]
pub struct Hidden {
    id: Uuid,
    /**
    Contains
    Vec<{input_type: Input | Hidden, weight}>
    */
    inputs: Vec<NeuronInput>,
    bias: f32,
    activation: fn(f32) -> f32,
}
#[derive(Clone)]
pub struct Output {
    id: Uuid,
    inputs: Vec<NeuronInput>,
    bias: f32,
    activation: fn(f32) -> f32,
}

impl Output {}

#[derive(Clone)]
pub struct NeuronInput {
    pub input_type: NeuronInputType,
    pub weight: f32,
}
#[derive(Clone)]
pub enum NeuronInputType {
    Input(Weak<Mutex<Input>>),
    Hidden(Weak<Mutex<Hidden>>),
}
impl From<&NeuronTopology<Input>> for NeuronInputType {
    fn from(value: &NeuronTopology<Input>) -> Self {
        Self::Input(Arc::downgrade(&value.inner))
    }
}
impl From<&NeuronTopology<Hidden>> for NeuronInputType {
    fn from(value: &NeuronTopology<Hidden>) -> Self {
        Self::Hidden(Arc::downgrade(&value.inner))
    }
}
impl TopologyNeuron for Hidden {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl TakesInput for Hidden {
    fn new_from_raw_parts(inputs: Vec<NeuronInput>, bias: f32, activation: fn(f32) -> f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            inputs,
            bias,
            activation,
        }
    }
    fn add_input(&mut self, input: impl Into<NeuronInputType>) {
        self.inputs.push(NeuronInput {
            input_type: input.into(),
            weight: 1.,
        })
    }

    fn inputs(&self) -> &[NeuronInput] {
        &self.inputs
    }
    fn bias(&self) -> f32 {
        self.bias
    }
    fn activation(&self) -> fn(f32) -> f32 {
        self.activation
    }
    fn mutate_random_weight(&mut self, rng: &mut impl Rng) {
        if let Some(input) = self.inputs.choose_mut(rng) {
            input.weight += rng.random_range(-1.0..=1.0);
        }
    }
}

impl TopologyNeuron for Output {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl TakesInput for Output {
    fn new_from_raw_parts(inputs: Vec<NeuronInput>, bias: f32, activation: fn(f32) -> f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            inputs,
            bias,
            activation,
        }
    }

    fn add_input(&mut self, input: impl Into<NeuronInputType>) {
        self.inputs.push(NeuronInput {
            input_type: input.into(),
            weight: 1.,
        })
    }

    fn inputs(&self) -> &[NeuronInput] {
        &self.inputs
    }

    fn bias(&self) -> f32 {
        self.bias
    }

    fn activation(&self) -> fn(f32) -> f32 {
        self.activation
    }

    fn mutate_random_weight(&mut self, rng: &mut impl Rng) {
        if let Some(input) = self.inputs.choose_mut(rng) {
            input.weight += rng.random_range(-1.0..=1.0);
        }
    }
}
