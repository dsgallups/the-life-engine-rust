pub trait Junction {
    /// The network's side that receives input
    fn afference(&mut self) -> Option<Vec<JunctionAffer>>;
    /// the network will push output here
    fn efference(&mut self) -> Option<Vec<JunctionEffer>>;
}

pub enum NeuronInterface {
    Junction(),
}

/// This receives input
pub struct JunctionAffer {
    // channel: <f32>,
    // cur_value: f32,
}

/// This receives input
pub struct JunctionEffer {
    //channel:Bus<f32>,
}
