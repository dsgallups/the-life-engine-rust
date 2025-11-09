use bevy::prelude::*;
use nora_neat::prelude::NetworkTopology;

#[derive(Clone)]
pub enum CellDetails {
    Brain(NetworkTopology),
    Launcher,
    Eye,
    Collagen,
    Data,
}
impl CellDetails {
    pub fn cell_type(&self) -> CellKind {
        match self {
            CellDetails::Brain(_) => CellKind::Brain,
            CellDetails::Collagen => CellKind::Collagen,
            CellDetails::Data => CellKind::Data,
            CellDetails::Eye => CellKind::Eye,
            CellDetails::Launcher => CellKind::Launcher,
        }
    }
}

#[derive(Component, Reflect, Clone, Copy, PartialEq, Eq)]
pub enum CellKind {
    Brain,
    Launcher,
    Eye,
    Collagen,
    Data,
}
