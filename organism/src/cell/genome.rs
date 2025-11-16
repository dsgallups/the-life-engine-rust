use bevy::prelude::*;

use crate::{
    CellKind,
    genome::{Input, NeuronTopology, Output},
};

#[derive(Debug, Clone)]
pub struct CellGenome {
    pub kind: CellKind,
    pub inputs: Vec<NeuronTopology<Input>>,
    pub outputs: Vec<NeuronTopology<Output>>,
}

// #[derive(Clone)]
// pub struct CellGenome {
//     id: Uuid,
//     kind: CellKind,
//     location: IVec2,
//     /// things I will input into the network
//     inputs: Vec<usize>,
//     /// indices I will read from the network
//     outputs: Vec<usize>,
// }
// impl CellGenome {
//     pub fn new(id: Uuid, kind: CellKind, location: IVec2) -> Self {
//         Self {
//             id,
//             kind,
//             location,
//             inputs: vec![],
//             outputs: vec![],
//         }
//     }
//     pub fn set_inputs(&mut self, inputs: Vec<usize>) {
//         self.inputs = inputs;
//     }
//     pub fn set_outputs(&mut self, outputs: Vec<usize>) {
//         self.outputs = outputs;
//     }

//     pub fn details(&self) -> &CellKind {
//         &self.kind
//     }
//     pub fn location(&self) -> IVec2 {
//         self.location
//     }
// }
