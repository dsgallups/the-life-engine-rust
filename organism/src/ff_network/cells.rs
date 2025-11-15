use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellKind {
    Launcher,
    Eye,
    Collagen,
    Data,
}
impl CellKind {
    pub fn requirements(&self) -> CellRequirements {
        use CellKind::*;
        let num_inputs = match self {
            Launcher => 0,
            Eye => 2,
            Collagen => 0,
            Data => 4,
        };
        let num_outputs = match self {
            Launcher => 3,
            Eye => 0,
            Collagen => 0,
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
