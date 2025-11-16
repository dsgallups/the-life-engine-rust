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
    // returns true if the input was an input of this type prior to removing it
    //fn remove_input(&mut self, input: &impl CanBeInput) -> Option<NeuronInput>;
    fn inputs(&self) -> &[NeuronInput];
    fn inputs_mut(&mut self) -> &mut Vec<NeuronInput>;

    fn bias(&self) -> f32;
    fn bias_mut(&mut self) -> &mut f32;

    fn activation(&self) -> fn(f32) -> f32;
    fn set_activation(&mut self, activation: fn(f32) -> f32);

    fn random_input<'a>(&mut self, rng: &'a mut impl Rng) -> Option<&mut NeuronInput> {
        self.inputs_mut().choose_mut(rng)
    }
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
impl NeuronInput {
    pub fn is_alive(&self) -> bool {
        match &self.input_type {
            NeuronInputType::Hidden(h) => h.upgrade().is_some(),
            NeuronInputType::Input(i) => i.upgrade().is_some(),
        }
    }

    pub fn id(&self) -> Option<Uuid> {
        match &self.input_type {
            NeuronInputType::Hidden(h) => {
                let h = h.upgrade()?;
                Some(h.id())
            }
            NeuronInputType::Input(i) => {
                let i = i.upgrade()?;
                Some(i.id())
            }
        }
    }
}

#[derive(Clone)]
pub struct NeuronInputInner<T>(Weak<Mutex<T>>);
impl<T> NeuronInputInner<T> {
    /// Attempts to upgrade this inner type to the associated neuron topology, if the link is still valid.
    pub fn upgrade(&self) -> Option<NeuronTopology<T>> {
        let upgraded = self.0.upgrade()?;
        Some(NeuronTopology::from_inner(upgraded))
    }
}

#[derive(Clone)]
pub enum NeuronInputType {
    Input(NeuronInputInner<Input>),
    Hidden(NeuronInputInner<Hidden>),
}
impl NeuronInputType {
    pub fn hidden(topology: &NeuronTopology<Hidden>) -> Self {
        Self::Hidden(NeuronInputInner(Arc::downgrade(&topology.inner)))
    }
    pub fn input(topology: &NeuronTopology<Input>) -> Self {
        Self::Input(NeuronInputInner(Arc::downgrade(&topology.inner)))
    }
}

impl CanBeInput for NeuronTopology<Input> {
    fn to_input_type(&self) -> NeuronInputType {
        NeuronInputType::Input(NeuronInputInner(Arc::downgrade(&self.inner)))
    }
    fn equals(&self, other: &NeuronInputType) -> bool {
        match other {
            NeuronInputType::Input(other) => {
                let Some(input) = other.upgrade() else {
                    return false;
                };
                Arc::ptr_eq(&self.inner, &input.inner)
            }
            NeuronInputType::Hidden(_) => false,
        }
    }
}
impl CanBeInput for NeuronTopology<Hidden> {
    fn to_input_type(&self) -> NeuronInputType {
        NeuronInputType::Hidden(NeuronInputInner(Arc::downgrade(&self.inner)))
    }
    fn equals(&self, other: &NeuronInputType) -> bool {
        match other {
            NeuronInputType::Hidden(other) => {
                let Some(input) = other.upgrade() else {
                    return false;
                };
                Arc::ptr_eq(&self.inner, &input.inner)
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
    fn inputs_mut(&mut self) -> &mut Vec<NeuronInput> {
        &mut self.inputs
    }
    // fn remove_input(&mut self, input_to_remove: &impl CanBeInput) -> Option<NeuronInput> {
    //     if let Some(position) = self
    //         .inputs
    //         .iter()
    //         .position(|input| input_to_remove.equals(&input.input_type))
    //     {
    //         Some(self.inputs.swap_remove(position))
    //     } else {
    //         None
    //     }
    // }

    fn inputs(&self) -> &[NeuronInput] {
        &self.inputs
    }
    fn bias(&self) -> f32 {
        self.bias
    }
    fn bias_mut(&mut self) -> &mut f32 {
        &mut self.bias
    }
    fn activation(&self) -> fn(f32) -> f32 {
        self.activation
    }
    fn set_activation(&mut self, activation: fn(f32) -> f32) {
        self.activation = activation;
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

    fn inputs_mut(&mut self) -> &mut Vec<NeuronInput> {
        &mut self.inputs
    }
    // fn remove_input(&mut self, input_to_remove: &impl CanBeInput) -> Option<NeuronInput> {
    //     if let Some(position) = self
    //         .inputs
    //         .iter()
    //         .position(|input| input_to_remove.equals(&input.input_type))
    //     {
    //         Some(self.inputs.swap_remove(position))
    //     } else {
    //         None
    //     }
    // }
    fn bias_mut(&mut self) -> &mut f32 {
        &mut self.bias
    }
    fn set_activation(&mut self, activation: fn(f32) -> f32) {
        self.activation = activation;
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
