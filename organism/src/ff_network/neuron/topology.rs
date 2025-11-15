use std::sync::{Arc, Mutex, MutexGuard};

use rand::Rng;
use uuid::Uuid;

use crate::ff_network::{
    CanBeInput, Hidden, Input, NeuronInput, Output, TakesInput, TopologyNeuron,
};

#[derive(Clone)]
pub struct NeuronTopology<Type> {
    /// None is an input node, Some is hidden or output
    pub inner: Arc<Mutex<Type>>,
}

impl<T> NeuronTopology<T> {
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.inner.lock().unwrap()
    }
    pub fn from_inner(inner: Arc<Mutex<T>>) -> Self {
        Self { inner }
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
    pub fn add_input(&self, input: &impl CanBeInput) {
        self.lock().add_input(input);
    }
    pub fn for_random_input<'rng, R, F, V>(&self, rng: &'rng mut R, func: F) -> Option<V>
    where
        R: Rng,
        F: for<'a> FnOnce(&'a mut NeuronInput, &'rng mut R) -> V,
    {
        let mut lock = self.lock();
        if let Some(rand_input) = lock.random_input(rng) {
            return Some(func(rand_input, rng));
        } else {
            return None;
        }
    }

    pub fn for_inputs<F, V>(&self, func: F) -> V
    where
        F: FnOnce(&mut Vec<NeuronInput>) -> V,
    {
        let mut lock = self.lock();
        let inputs = lock.inputs_mut();
        func(inputs)
    }

    // /// returns true if the input existed prior to this operation
    // pub fn remove_input(&self, input: &impl CanBeInput) -> Option<NeuronInput> {
    //     self.lock().remove_input(input)
    // }
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
