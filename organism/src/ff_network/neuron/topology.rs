use std::sync::{Arc, Mutex, MutexGuard};

use uuid::Uuid;

use crate::ff_network::{Hidden, Input, NeuronInput, NeuronInputType, Output, TakesInput};

#[derive(Clone)]
pub struct NeuronTopology<Type> {
    /// None is an input node, Some is hidden or output
    pub inner: Arc<Mutex<Inner<Type>>>,
}

impl<T> NeuronTopology<T> {
    pub fn lock(&self) -> MutexGuard<'_, Inner<T>> {
        self.inner.lock().unwrap()
    }
}

impl NeuronTopology<Input> {
    pub fn input() -> Self {
        let inner = Inner {
            id: Uuid::new_v4(),
            n_type: Input,
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
            n_type: Hidden {
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
    pub fn add_input(&mut self, input: impl Into<NeuronInputType>) {
        let mut inner = self.inner.lock().unwrap();
        let self_inputs = &mut inner.n_type;
        self_inputs.add_input(input);
    }
}
impl NeuronTopology<Output> {
    pub fn output() -> Self {
        let inner = Inner {
            id: Uuid::new_v4(),
            n_type: Output {
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
pub struct Inner<Type> {
    pub id: Uuid,
    pub n_type: Type,
}
impl<T> Inner<T> {
    pub fn id(&self) -> Uuid {
        self.id
    }
}
impl<T: TakesInput> TakesInput for Inner<T> {
    fn add_input(&mut self, input: impl Into<NeuronInputType>) {
        self.n_type.add_input(input);
    }
    fn inputs(&self) -> &[NeuronInput] {
        self.n_type.inputs()
    }
}
