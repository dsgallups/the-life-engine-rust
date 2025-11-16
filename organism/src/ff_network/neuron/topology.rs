use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use rand::Rng;
use uuid::Uuid;

use crate::ff_network::{
    CanBeInput, Hidden, Input, NeuronInput, Output, TakesInput, TopologyNeuron,
};

#[derive(Clone, Debug)]
pub struct NeuronTopology<Type> {
    /// None is an input node, Some is hidden or output
    pub inner: Arc<RwLock<Type>>,
}

impl<T> NeuronTopology<T> {
    pub fn new(neuron_type: T) -> Self {
        Self::from_inner(Arc::new(RwLock::new(neuron_type)))
    }
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.inner.read().unwrap()
    }
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.inner.write().unwrap()
    }
    pub fn from_inner(inner: Arc<RwLock<T>>) -> Self {
        Self { inner }
    }
}

impl NeuronTopology<Input> {
    pub fn input() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Input::default())),
        }
    }
}
impl NeuronTopology<Hidden> {
    pub fn hidden() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Hidden::new_from_raw_parts(
                Vec::new(),
                0.,
                |_| 0.,
            ))),
        }
    }
}
impl<T: TopologyNeuron> NeuronTopology<T> {
    pub fn id(&self) -> Uuid {
        self.read().id()
    }
}

impl<T: TakesInput> NeuronTopology<T> {
    pub fn add_input(&self, input: &impl CanBeInput) {
        self.write().add_input(input);
    }
    pub fn for_random_input<'rng, R, F, V>(&self, rng: &'rng mut R, func: F) -> Option<V>
    where
        R: Rng,
        F: FnOnce(&mut NeuronInput, &'rng mut R) -> V,
    {
        self.with_mut(|lock| {
            lock.random_input(rng)
                .map(|rand_input| func(rand_input, rng))
        })
    }

    pub fn with_ref<F, V>(&self, func: F) -> V
    where
        F: FnOnce(&T) -> V,
    {
        let lock = self.read();
        func(&*lock)
    }

    pub fn with_mut<F, V>(&self, func: F) -> V
    where
        F: FnOnce(&mut T) -> V,
    {
        let mut lock = self.write();
        func(&mut *lock)
    }
}
impl NeuronTopology<Output> {
    pub fn output() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Output::new_from_raw_parts(
                Vec::new(),
                0.,
                |_| 0.,
            ))),
        }
    }
}
