use std::sync::{Arc, RwLock};

use crate::naive_net::neuron::SimpleNeuron;

pub struct Active(Arc<RwLock<SimpleNeuron>>);
impl Active {
    pub fn new(inner: Arc<RwLock<SimpleNeuron>>) -> Self {
        Self(inner)
    }
    pub fn handle(&self) -> &Arc<RwLock<SimpleNeuron>> {
        &self.0
    }
}
