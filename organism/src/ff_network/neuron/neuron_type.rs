use crate::ff_network::NeuronTopology;

use super::Inner;
use std::sync::{Arc, Mutex, Weak};

#[derive(Clone)]
pub struct Input;

#[derive(Clone)]
pub struct Hidden {
    pub(super) inputs: Vec<NeuronInputTop>,
    pub(super) bias: f32,
    pub(super) activation: fn(f32) -> f32,
}
#[derive(Clone)]
pub struct Output {
    pub(super) inputs: Vec<NeuronInputTop>,
    pub(super) bias: f32,
    pub(super) activation: fn(f32) -> f32,
}

#[derive(Clone)]
pub struct NeuronInputTop {
    neuron: NeuronInputTopNeuron,
    weight: f32,
}
#[derive(Clone)]
pub enum NeuronInputTopNeuron {
    Input(Weak<Mutex<Inner<Input>>>),
    Hidden(Weak<Mutex<Inner<Hidden>>>),
}
impl From<&NeuronTopology<Input>> for NeuronInputTopNeuron {
    fn from(value: &NeuronTopology<Input>) -> Self {
        Self::Input(Arc::downgrade(&value.inner))
    }
}
impl From<&NeuronTopology<Hidden>> for NeuronInputTopNeuron {
    fn from(value: &NeuronTopology<Hidden>) -> Self {
        Self::Hidden(Arc::downgrade(&value.inner))
    }
}

pub trait TakesInput {
    fn add_input(&mut self, input: impl Into<NeuronInputTopNeuron>);
}
impl TakesInput for Hidden {
    fn add_input(&mut self, input: impl Into<NeuronInputTopNeuron>) {
        self.inputs.push(NeuronInputTop {
            neuron: input.into(),
            weight: 1.,
        })
    }
}

impl TakesInput for Output {
    fn add_input(&mut self, input: impl Into<NeuronInputTopNeuron>) {
        self.inputs.push(NeuronInputTop {
            neuron: input.into(),
            weight: 1.,
        })
    }
}
