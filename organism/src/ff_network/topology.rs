use std::sync::{Arc, Mutex};

use uuid::Uuid;

#[derive(Clone)]
pub struct NeuronTopology {
    /// None is an input node, Some is hidden or output
    inner: Arc<Mutex<Inner>>,
}

impl NeuronTopology {
    pub fn input() -> Self {
        let inner = Inner {
            id: Uuid::new_v4(),
            inputs: None,
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
    pub fn output() -> Self {
        let inner = Inner {
            id: Uuid::new_v4(),
            inputs: Some(Inputs {
                int_type: IntermediateType::Output,
                inputs: Vec::new(),
                bias: 0.,
                activation: |_| 0.,
            }),
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
    /// will panic if this is not intermediary. This is only
    /// for building.
    pub fn set_initial_inputs(&mut self, inputs: Vec<NeuronTopology>) {
        let mut inner = self.inner.lock().unwrap();
        let self_inputs = inner.inputs.as_mut().unwrap();
        self_inputs.inputs = inputs
            .into_iter()
            .map(|neuron| NeuronInputTop { neuron, weight: 1. })
            .collect();
    }
}
struct Inner {
    id: Uuid,
    inputs: Option<Inputs>,
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

pub struct NeuronInputTop {
    neuron: NeuronTopology,
    weight: f32,
}
