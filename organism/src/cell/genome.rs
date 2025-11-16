use bevy::prelude::*;
use uuid::Uuid;

use crate::CellDetails;

#[derive(Clone)]
pub struct CellGenome {
    id: Uuid,
    details: CellDetails,
    location: IVec2,
    /// things I will input into the network
    inputs: Vec<usize>,
    /// indices I will read from the network
    outputs: Vec<usize>,
}
impl CellGenome {
    pub fn new(id: Uuid, kind: CellDetails, location: IVec2) -> Self {
        Self {
            id,
            details: kind,
            location,
            inputs: vec![],
            outputs: vec![],
        }
    }
    pub fn set_inputs(&mut self, inputs: Vec<usize>) {
        self.inputs = inputs;
    }
    pub fn set_outputs(&mut self, outputs: Vec<usize>) {
        self.outputs = outputs;
    }

    pub fn details(&self) -> &CellDetails {
        &self.details
    }
    pub fn location(&self) -> IVec2 {
        self.location
    }
}
