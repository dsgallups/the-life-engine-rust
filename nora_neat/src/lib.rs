//pub mod burn_net;
pub mod naive_net;

pub mod activation;
pub mod active;
pub mod mutation;
pub mod network;
pub mod neuron;

mod test_utils;

pub mod prelude {
    pub use crate::activation::*;
    pub use crate::mutation::*;
    pub use crate::network::*;
    pub use crate::neuron::*;
    pub(crate) use crate::test_utils::*;
}

// #[cfg(test)]
// mod tests;
