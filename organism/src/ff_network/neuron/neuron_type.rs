use crate::ff_network::NeuronTopology;

use super::Inner;
use std::sync::{Arc, Mutex, Weak};

#[derive(Clone)]
pub struct Input;

#[derive(Clone)]
pub struct Hidden {
    /**
    Contains
    Vec<{input_type: Input | Hidden, weight}>
    */
    pub inputs: Vec<NeuronInput>,
    pub bias: f32,
    pub activation: fn(f32) -> f32,
}
#[derive(Clone)]
pub struct Output {
    pub inputs: Vec<NeuronInput>,
    pub bias: f32,
    pub activation: fn(f32) -> f32,
}

impl Output {}

#[derive(Clone)]
pub struct NeuronInput {
    pub input_type: NeuronInputType,
    pub weight: f32,
}
#[derive(Clone)]
pub enum NeuronInputType {
    Input(Weak<Mutex<Inner<Input>>>),
    Hidden(Weak<Mutex<Inner<Hidden>>>),
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

pub trait TakesInput {
    fn add_input(&mut self, input: impl Into<NeuronInputType>);
    fn inputs(&self) -> &[NeuronInput];
}
impl TakesInput for Hidden {
    fn add_input(&mut self, input: impl Into<NeuronInputType>) {
        self.inputs.push(NeuronInput {
            input_type: input.into(),
            weight: 1.,
        })
    }

    fn inputs(&self) -> &[NeuronInput] {
        &self.inputs
    }
}

impl TakesInput for Output {
    fn add_input(&mut self, input: impl Into<NeuronInputType>) {
        self.inputs.push(NeuronInput {
            input_type: input.into(),
            weight: 1.,
        })
    }

    fn inputs(&self) -> &[NeuronInput] {
        &self.inputs
    }
}
