use std::sync::{Arc, RwLock, Weak};

use crate::prelude::*;
use rand::Rng;

/// Represents a weighted input connection in a polynomial neural network.
///
/// Each `PolyInput` encapsulates:
/// - The source of the input (typically a neuron identifier)
/// - The connection weight
/// - The exponent applied to the input value
#[derive(Clone, Debug)]
pub struct NeuronInput<I> {
    input: I,
    weight: f32,
    exp: i32,
}

impl<I> NeuronInput<I> {
    /// Creates a new `PolyInput` with specified parameters.
    pub fn new(input: I, weight: f32, exp: i32) -> Self {
        Self { input, weight, exp }
    }

    /// Creates a new `PolyInput` with random weight and exponent.
    pub fn new_rand(input: I, rng: &mut impl Rng) -> Self {
        Self {
            input,
            weight: rng.random_range(-1.0..=1.0),
            exp: rng.random_range(0..=2),
        }
    }

    /// Returns a reference to the input identifier.
    pub fn input(&self) -> &I {
        &self.input
    }

    /// Returns the connection weight.
    pub fn weight(&self) -> f32 {
        self.weight
    }

    /// Adjusts the connection weight by adding the specified delta.
    pub fn adjust_weight(&mut self, by: f32) {
        self.weight += by;
    }

    /// Returns the exponent applied to the input value.
    pub fn exponent(&self) -> i32 {
        self.exp
    }

    /// Adjusts the exponent by adding the specified delta.
    pub fn adjust_exp(&mut self, by: i32) {
        self.exp += by;
    }
}

impl NeuronInput<Topology> {
    pub fn neuron(&self) -> Option<Arc<RwLock<NeuronTopology>>> {
        Weak::upgrade(self.input().handle())
    }

    pub fn downgrade(input: &Arc<RwLock<NeuronTopology>>, weight: f32, exp: i32) -> Self {
        Self::new(Topology::new(input), weight, exp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_new() {
        let input = NeuronInput::new(42, 0.5, 2);
        assert_eq!(*input.input(), 42);
        assert_eq!(input.weight(), 0.5);
        assert_eq!(input.exponent(), 2);
    }

    #[test]
    fn test_new_with_negative_weight() {
        let input = NeuronInput::new("test", -0.75, 0);
        assert_eq!(*input.input(), "test");
        assert_eq!(input.weight(), -0.75);
        assert_eq!(input.exponent(), 0);
    }

    #[test]
    fn test_new_rand_ranges() {
        let mut rng = StdRng::seed_from_u64(42);

        // Test multiple random generations to ensure ranges are respected
        for _ in 0..100 {
            let input = NeuronInput::new_rand(1, &mut rng);
            assert!(
                input.weight() >= -1.0 && input.weight() <= 1.0,
                "Weight {} should be in range [-1.0, 1.0]",
                input.weight()
            );
            assert!(
                input.exponent() >= 0 && input.exponent() <= 2,
                "Exponent {} should be in range [0, 2]",
                input.exponent()
            );
        }
    }

    #[test]
    fn test_new_rand_distribution() {
        let mut rng = StdRng::seed_from_u64(12345);
        let num_samples = 1000;

        let mut weight_sum = 0.0;
        let mut exp_counts = [0; 3]; // For exponents 0, 1, 2

        for _ in 0..num_samples {
            let input = NeuronInput::new_rand(1, &mut rng);
            weight_sum += input.weight();
            exp_counts[input.exponent() as usize] += 1;
        }

        // Check weight distribution (should average near 0)
        let weight_mean = weight_sum / num_samples as f32;
        assert!(
            weight_mean.abs() < 0.1,
            "Weight mean {} should be close to 0",
            weight_mean
        );

        // Check exponent distribution (should be roughly uniform)
        for (exp, count) in exp_counts.iter().enumerate() {
            let ratio = *count as f32 / num_samples as f32;
            assert!(
                (ratio - 0.333).abs() < 0.05,
                "Exponent {} ratio {} should be close to 0.333",
                exp,
                ratio
            );
        }
    }

    #[test]
    fn test_adjust_weight() {
        let mut input = NeuronInput::new(1, 0.5, 1);

        input.adjust_weight(0.3);
        assert!((input.weight() - 0.8).abs() < f32::EPSILON);

        input.adjust_weight(-0.5);
        assert!((input.weight() - 0.3).abs() < f32::EPSILON);

        input.adjust_weight(-0.3);
        assert!((input.weight() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_adjust_exp() {
        let mut input = NeuronInput::new(1, 0.5, 1);

        input.adjust_exp(2);
        assert_eq!(input.exponent(), 3);

        input.adjust_exp(-1);
        assert_eq!(input.exponent(), 2);

        input.adjust_exp(-2);
        assert_eq!(input.exponent(), 0);

        // Test negative exponents
        input.adjust_exp(-1);
        assert_eq!(input.exponent(), -1);
    }

    #[test]
    fn test_clone() {
        let original = NeuronInput::new(42, 0.7, 2);
        let cloned = original.clone();

        assert_eq!(*cloned.input(), *original.input());
        assert_eq!(cloned.weight(), original.weight());
        assert_eq!(cloned.exponent(), original.exponent());
    }

    #[test]
    fn test_debug_format() {
        let input = NeuronInput::new(123, 0.5, 1);
        let debug_str = format!("{:?}", input);

        assert!(debug_str.contains("PolyInput"));
        assert!(debug_str.contains("123"));
        assert!(debug_str.contains("0.5"));
        assert!(debug_str.contains("1"));
    }

    #[test]
    fn test_deterministic_rand() {
        let seed = 9876;
        let mut rng1 = StdRng::seed_from_u64(seed);
        let mut rng2 = StdRng::seed_from_u64(seed);

        for i in 0..10 {
            let input1 = NeuronInput::new_rand(i, &mut rng1);
            let input2 = NeuronInput::new_rand(i, &mut rng2);

            assert_eq!(input1.weight(), input2.weight());
            assert_eq!(input1.exponent(), input2.exponent());
        }
    }

    #[test]
    fn test_polynomial_calculation_example() {
        // Example showing how the polynomial input would be used
        let input = NeuronInput::new(1, 0.5, 2);
        let input_value = 3.0_f32;

        // Calculate contribution: weight * input_value^exponent
        let contribution = input.weight() * input_value.powi(input.exponent());
        let expected = 0.5 * 3.0_f32.powi(2); // 0.5 * 9 = 4.5

        assert!((contribution - expected).abs() < f32::EPSILON);
    }

    #[test]
    fn test_with_different_input_types() {
        // Test with string IDs
        let string_input = NeuronInput::new("neuron-a", 0.5, 1);
        assert_eq!(*string_input.input(), "neuron-a");

        // Test with usize IDs
        let usize_input = NeuronInput::new(42usize, 0.5, 1);
        assert_eq!(*usize_input.input(), 42usize);

        // Test with custom type
        #[derive(Debug, Clone, PartialEq)]
        struct NeuronId(u64);

        let custom_input = NeuronInput::new(NeuronId(123), 0.5, 1);
        assert_eq!(*custom_input.input(), NeuronId(123));
    }
}
