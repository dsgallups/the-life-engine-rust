use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    //todo
}

#[derive(Clone, Debug, PartialEq)]
pub struct Genome {
    cells: Vec<Cell>,
    mutation_rate: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell {
    cell_type: CellType,
    /// this is relative
    location: IVec2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellType {
    Armor,
    Eye,
    Killer,
    Mover,
    Producer,
    Mouth,
}
