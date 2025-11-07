use crate::naive_net::neuron::NaiveNeuron;

pub struct Active(NaiveNeuron);
impl Active {
    pub fn new(inner: NaiveNeuron) -> Self {
        Self(inner)
    }
    pub fn handle(&self) -> &NaiveNeuron {
        &self.0
    }
}
