use rand::Rng;

pub fn random_bias(rng: &mut impl Rng) -> f32 {
    rng.random_range(-1_f32..=1_f32)
}
pub fn random_activation(rng: &mut impl Rng) -> fn(f32) -> f32 {
    match rng.random_range(0..3) {
        0 => sigmoid,
        1 => relu,
        _ => linear_activation,
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
