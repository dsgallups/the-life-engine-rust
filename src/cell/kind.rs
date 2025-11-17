use bevy::prelude::*;
use strum::EnumIter;

#[derive(Component, Reflect, Clone, Copy, PartialEq, Eq, Hash, EnumIter, Debug)]
pub enum CellKind {
    Launcher,
    Eye,
    Foot,
    Data,
}
impl CellKind {
    pub fn requirements(&self) -> CellRequirements {
        use CellKind::*;
        let num_inputs = match self {
            Launcher => 0,
            Eye => 3,
            Foot => 0,
            Data => 4,
        };
        let num_outputs = match self {
            Launcher => 3,
            Eye => 0,
            Foot => 3,
            Data => 4,
        };
        CellRequirements {
            num_inputs,
            num_outputs,
        }
    }
}

pub struct CellRequirements {
    pub num_inputs: usize,
    pub num_outputs: usize,
}
