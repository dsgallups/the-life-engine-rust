use rand::{Rng, seq::IteratorRandom};
use strum::{EnumIter, IntoEnumIterator};

pub struct Bias;

impl Bias {
    /// Generates a random bias value in the range [0, 1).
    pub fn rand(rng: &mut impl Rng) -> f32 {
        rng.random()
    }
}

#[derive(Debug, EnumIter)]
pub enum Activation {
    Sigmoid,
    Relu,
    Linear,
}

impl Activation {
    pub fn rand(rng: &mut impl Rng) -> fn(f32) -> f32 {
        let random_val = Self::iter().choose(rng).unwrap();
        match random_val {
            Activation::Linear => linear_activation,
            Activation::Relu => relu,
            Activation::Sigmoid => sigmoid,
        }
    }
}

pub fn sigmoid(n: f32) -> f32 {
    1. / (1. + std::f32::consts::E.powf(-n))
}

pub fn relu(n: f32) -> f32 {
    n.max(0.)
}

pub fn linear_activation(n: f32) -> f32 {
    n
}
