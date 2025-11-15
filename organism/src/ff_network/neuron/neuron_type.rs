use rand::{Rng, seq::IndexedMutRandom};
use uuid::Uuid;

use crate::ff_network::NeuronTopology;

use std::sync::{Arc, Mutex, Weak};

pub trait TopologyNeuron {
    fn id(&self) -> Uuid;
}

pub trait CanBeInput {
    fn to_input_type(&self) -> NeuronInputType;

    fn equals(&self, other: &NeuronInputType) -> bool;
}

pub trait TakesInput: TopologyNeuron {
    fn new_from_raw_parts(inputs: Vec<NeuronInput>, bias: f32, activation: fn(f32) -> f32) -> Self;
    fn add_input(&mut self, input: &impl CanBeInput);
    /// returns true if the input was an input of this type prior to removing it
    fn remove_input(&mut self, input: &impl CanBeInput) -> bool;
    fn inputs(&self) -> &[NeuronInput];
    fn bias(&self) -> f32;
    fn activation(&self) -> fn(f32) -> f32;
    fn random_input<'a>(&mut self, rng: &'a mut impl Rng) -> Option<&mut NeuronInput>;
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

impl CanBeInput for NeuronTopology<Input> {
    fn to_input_type(&self) -> NeuronInputType {
        NeuronInputType::Input(Arc::downgrade(&self.inner))
    }
    fn equals(&self, other: &NeuronInputType) -> bool {
        match other {
            NeuronInputType::Input(other) => {
                let Some(input) = other.upgrade() else {
                    return false;
                };
                Arc::ptr_eq(&self.inner, &input)
            }
            NeuronInputType::Hidden(_) => false,
        }
    }
}
impl CanBeInput for NeuronTopology<Hidden> {
    fn to_input_type(&self) -> NeuronInputType {
        NeuronInputType::Hidden(Arc::downgrade(&self.inner))
    }
    fn equals(&self, other: &NeuronInputType) -> bool {
        match other {
            NeuronInputType::Hidden(other) => {
                let Some(input) = other.upgrade() else {
                    return false;
                };
                Arc::ptr_eq(&self.inner, &input)
            }
            NeuronInputType::Input(_) => false,
        }
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
    fn add_input(&mut self, input: &impl CanBeInput) {
        self.inputs.push(NeuronInput {
            input_type: input.to_input_type(),
            weight: 1.,
        })
    }
    fn remove_input(&mut self, input_to_remove: &impl CanBeInput) -> bool {
        if let Some(position) = self
            .inputs
            .iter()
            .position(|input| input_to_remove.equals(&input.input_type))
        {
            self.inputs.swap_remove(position);
            true
        } else {
            false
        }
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
    fn random_input(&mut self, rng: &mut impl Rng) -> Option<&mut NeuronInput> {
        self.inputs.choose_mut(rng)

        // if let Some(input) = self.inputs.choose_mut(rng) {
        //     input.weight += rng.random_range(-1.0..=1.0);
        // }
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

    fn add_input(&mut self, input: &impl CanBeInput) {
        self.inputs.push(NeuronInput {
            input_type: input.to_input_type(),
            weight: 1.,
        })
    }
    fn remove_input(&mut self, input_to_remove: &impl CanBeInput) -> bool {
        if let Some(position) = self
            .inputs
            .iter()
            .position(|input| input_to_remove.equals(&input.input_type))
        {
            self.inputs.swap_remove(position);
            true
        } else {
            false
        }
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

    fn random_input(&mut self, rng: &mut impl Rng) -> Option<&mut NeuronInput> {
        self.inputs.choose_mut(rng)

        // if let Some(input) = self.inputs.choose_mut(rng) {
        //     input.weight += rng.random_range(-1.0..=1.0);
        // }
    }
}
