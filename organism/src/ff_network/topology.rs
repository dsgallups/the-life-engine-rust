use std::sync::{Arc, Mutex};

use uuid::Uuid;

#[derive(Clone)]
pub struct Input;

#[derive(Clone)]
pub struct Hidden {
    inputs: Vec<NeuronInputTop>,
    bias: f32,
    activation: fn(f32) -> f32,
}
#[derive(Clone)]
pub struct Output {
    inputs: Vec<NeuronInputTop>,
    bias: f32,
    activation: fn(f32) -> f32,
}

#[derive(Clone)]
pub struct NeuronTopology<Type> {
    /// None is an input node, Some is hidden or output
    inner: Arc<Mutex<Inner<Type>>>,
}

impl NeuronTopology<Input> {
    pub fn input() -> Self {
        let inner = Inner {
            id: Uuid::new_v4(),
            inputs: Input,
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}
impl NeuronTopology<Output> {
    pub fn output() -> Self {
        let inner = Inner {
            id: Uuid::new_v4(),
            inputs: Output {
                inputs: Vec::new(),
                bias: 0.,
                activation: |_| 0.,
            },
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
    /// will panic if this is not intermediary. This is only
    /// for building.
    pub fn add_input(&mut self, input: impl Into<NeuronInputTopNeuron>) {
        let mut inner = self.inner.lock().unwrap();
        let self_inputs = &mut inner.inputs;
        self_inputs.inputs.push(NeuronInputTop {
            neuron: input.into(),
            weight: 1.,
        });
    }
}
struct Inner<Type> {
    id: Uuid,
    inputs: Type,
}
struct Inputs {
    int_type: IntermediateType,
    inputs: Vec<NeuronInputTop>,
    bias: f32,
    activation: fn(f32) -> f32,
}
enum IntermediateType {
    Hidden,
    Output,
}

#[derive(Clone)]
pub struct NeuronInputTop {
    neuron: NeuronInputTopNeuron,
    weight: f32,
}
#[derive(Clone)]
pub enum NeuronInputTopNeuron {
    Input(NeuronTopology<Input>),
    Hidden(NeuronTopology<Hidden>),
}
impl From<NeuronTopology<Input>> for NeuronInputTopNeuron {
    fn from(value: NeuronTopology<Input>) -> Self {
        Self::Input(value)
    }
}
impl From<NeuronTopology<Hidden>> for NeuronInputTopNeuron {
    fn from(value: NeuronTopology<Hidden>) -> Self {
        Self::Hidden(value)
    }
}
