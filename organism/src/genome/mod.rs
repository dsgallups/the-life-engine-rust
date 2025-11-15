/**
Goals:

1. Feed forward (to keep it simple)
  - Will refactor in the future to use a markov brain. Just need
    it to work first.
2. Inputs are cells, outputs are cells. it's a loopdyloop
*/
use std::sync::Arc;

pub struct NeuronTopology {}

pub struct CellTemplate {
    inputs: Vec<Arc<NeuronTopology>>,
    outputs: Vec<Arc<NeuronTopology>>,
}
