use std::sync::{Arc, Mutex, MutexGuard};

use rand::Rng;
use uuid::Uuid;

use crate::ff_network::{Hidden, Input, NeuronInputType, Output, TakesInput, TopologyNeuron};

#[derive(Clone)]
pub struct NeuronTopology<Type> {
    /// None is an input node, Some is hidden or output
    pub inner: Arc<Mutex<Type>>,
}

impl<T> NeuronTopology<T> {
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.inner.lock().unwrap()
    }
}

impl NeuronTopology<Input> {
    pub fn input() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Input::default())),
        }
    }
}
impl NeuronTopology<Hidden> {
    pub fn hidden() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Hidden::new_from_raw_parts(
                Vec::new(),
                0.,
                |_| 0.,
            ))),
        }
    }
}
impl<T: TopologyNeuron> NeuronTopology<T> {
    pub fn id(&self) -> Uuid {
        self.lock().id()
    }
}

impl<T: TakesInput> NeuronTopology<T> {
    pub fn add_input(&self, input: impl Into<NeuronInputType>) {
        self.lock().add_input(input);
    }
    pub fn mutate_random_weight(&self, rng: &mut impl Rng) {
        self.lock().mutate_random_weight(rng);
    }
}
impl NeuronTopology<Output> {
    pub fn output() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Output::new_from_raw_parts(
                Vec::new(),
                0.,
                |_| 0.,
            ))),
        }
    }
}
