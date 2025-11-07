use std::sync::{Arc, RwLock};

use crate::naive_net::neuron::NaiveNeuron;

pub struct Active(Arc<RwLock<NaiveNeuron>>);
impl Active {
    pub fn new(inner: Arc<RwLock<NaiveNeuron>>) -> Self {
        Self(inner)
    }
    pub fn handle(&self) -> &Arc<RwLock<NaiveNeuron>> {
        &self.0
    }
}
