use std::f32;

use crate::{naive_net::neuron_type::Active, prelude::*};

impl NeuronInput<Active> {
    /// applies a weight and exponent to the input neuron and returns the result
    pub fn get_input_value(&self) -> f32 {
        //note that this can totally return inf if the value is zero. This is intentional.

        if let Some(cached) = {
            self.input()
                .handle()
                .inner()
                .read()
                .unwrap()
                .check_activated()
                .map(|val| val * self.weight())
        } {
            return cached;
        }

        let neuron_value = self.input().handle().inner().write().unwrap().activate();

        // if result.is_infinite() || result.is_nan() {
        //     println!(
        //         "DEBUG: Fresh calculation produced inf/nan: neuron_value={}, exp={}, weight={}, result={}",
        //         neuron_value,
        //         self.exponent(),
        //         self.weight(),
        //         result
        //     );
        // }
        // result

        neuron_value * self.weight()
    }
}

// #[test]
// fn ensure_inf_possible() {
//     use uuid::Uuid;
//     let neuron = Arc::new(RwLock::new(SimpleNeuron::new(
//         Uuid::new_v4(),
//         Some(NeuronPropsAlias::output(vec![NeuronInput::new(
//             Arc::new(RwLock::new(SimpleNeuron::new(Uuid::new_v4(), None))),
//             0.,
//             1,
//         )])),
//     )));
//     let neuron_input = NeuronInput::new(neuron, 1., -1);

//     let value = neuron_input.get_input_value();
//     assert_eq!(value, f32::INFINITY);
// }
// #[test]
// fn zero_to_zero_power() {
//     use uuid::Uuid;
//     let neuron = Arc::new(RwLock::new(SimpleNeuron::new(
//         Uuid::new_v4(),
//         Some(NeuronPropsAlias::output(vec![NeuronInput::new(
//             Arc::new(RwLock::new(SimpleNeuron::new(Uuid::new_v4(), None))),
//             0.,
//             1,
//         )])),
//     )));
//     let neuron_input = NeuronInput::new(neuron, 1., 0);

//     let value = neuron_input.get_input_value();
//     assert_eq!(value, 1.);
// }
