use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::ff_network::{Hidden, Input, NeuronInputTopNeuron, Output, TakesInput};

#[derive(Clone)]
pub struct NeuronTopology<Type> {
    /// None is an input node, Some is hidden or output
    pub(super) inner: Arc<Mutex<Inner<Type>>>,
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
impl NeuronTopology<Hidden> {
    pub fn hidden() -> Self {
        let inner = Inner {
            id: Uuid::new_v4(),
            inputs: Hidden {
                inputs: Vec::new(),
                bias: 0.,
                activation: |_| 0.,
            },
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}

impl<T: TakesInput> NeuronTopology<T> {
    pub fn add_input(&mut self, input: impl Into<NeuronInputTopNeuron>) {
        let mut inner = self.inner.lock().unwrap();
        let self_inputs = &mut inner.inputs;
        self_inputs.add_input(input);
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
}
pub(super) struct Inner<Type> {
    id: Uuid,
    inputs: Type,
}
