use rand::Rng;
//use strum::EnumIter;

pub struct Bias;

impl Bias {
    /// Generates a random bias value in the range [0, 1).
    pub fn rand(rng: &mut impl Rng) -> f32 {
        rng.random()
    }
}

// pub struct Exponent;

// impl Exponent {
//     pub fn rand(rng: &mut impl Rng) -> i32 {
//         rng.random_range(0..=1)
//     }
// }

#[derive(Debug)]
pub enum Activation {
    Sigmoid,
    Relu,
    Linear,
}

impl Activation {
    pub fn rand(rng: &mut impl Rng) -> fn(f32) -> f32 {
        //let random_val = Self::iter().choose(&mut rng).unwrap();

        todo!()
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
