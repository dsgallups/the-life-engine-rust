//pub mod burn_net;
//pub mod simple_net;

pub mod activation;
pub mod mutation;
pub mod network;
pub mod neuron;
pub mod topology;

mod test_utils;

pub mod prelude {
    pub use crate::activation::*;
    pub use crate::mutation::*;
    pub use crate::network::*;
    pub use crate::neuron::*;
    pub(crate) use crate::test_utils::*;
    pub use crate::topology::*;
}

// #[cfg(test)]
// mod tests;
