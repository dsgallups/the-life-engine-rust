use std::sync::{Arc, RwLock};

pub struct CpuNeuronInputs {
    pub(crate) inputs: Vec<(CpuNeuron, f32)>,
    pub bias: f32,
    pub activation: fn(f32) -> f32,
}

pub struct CpuNeuronInner {
    pub inputs: Option<CpuNeuronInputs>,
    pub value: Option<f32>,
}

#[derive(Clone)]
pub struct CpuNeuron {
    pub(super) inner: Arc<RwLock<CpuNeuronInner>>,
}

impl CpuNeuron {
    pub fn input() -> Self {
        Self {
            inner: Arc::new(RwLock::new(CpuNeuronInner {
                inputs: None,
                value: None,
            })),
        }
    }
    pub fn propagate_reset(&self) {
        {
            let read_lock = self.inner.read().unwrap();
            if read_lock.value.is_none() {
                return;
            }
            if let Some(inputs) = &read_lock.inputs {
                for (neuron, _) in &inputs.inputs {
                    neuron.propagate_reset();
                }
            }
        }

        {
            let mut write_lock = self.inner.write().unwrap();
            write_lock.value = None
        }
    }
    pub fn process(&self) -> f32 {
        let determined_value = {
            let read_lock = self.inner.read().unwrap();
            if let Some(value) = read_lock.value {
                return value;
            }

            let mut running_sum = 0.;

            let Some(neuron_inputs) = &read_lock.inputs else {
                return running_sum;
            };

            for (neuron, weight) in &neuron_inputs.inputs {
                let value = neuron.process();
                running_sum += value * *weight;
            }

            (neuron_inputs.activation)(running_sum) + neuron_inputs.bias
        };

        {
            let mut write_lock = self.inner.write().unwrap();
            write_lock.value = Some(determined_value);
        }

        determined_value
    }
}
