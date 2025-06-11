use bevy::prelude::*;

use crate::gameplay::cell::CellType;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CellGenome>()
        .register_type::<OrganismGenome>();
    app.init_asset::<OrganismGenome>();
    //todo
}

#[derive(Asset, Reflect, Clone, Debug, PartialEq)]
pub struct OrganismGenome {
    cells: Vec<CellGenome>,
    mutation_rate: f64,
}

impl OrganismGenome {
    fn new(cells: Vec<CellGenome>, mutation_rate: f64) -> Self {
        Self {
            cells,
            mutation_rate,
        }
    }
    pub fn first_organism() -> Self {
        Self::new(
            vec![
                CellGenome::new(CellType::Producer, IVec2::new(-1, -1)),
                CellGenome::new(CellType::Mouth, IVec2::new(0, 0)),
                CellGenome::new(CellType::Producer, IVec2::new(1, 1)),
            ],
            50.,
        )
    }
    pub fn iter_cells(&self) -> std::slice::Iter<'_, CellGenome> {
        self.cells.iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub struct CellGenome {
    cell_type: CellType,
    /// this is relative
    location: IVec2,
}
impl CellGenome {
    pub fn new(cell_type: CellType, location: IVec2) -> Self {
        Self {
            cell_type,
            location,
        }
    }
    pub fn cell_type(&self) -> CellType {
        self.cell_type
    }
    pub fn location(&self) -> IVec2 {
        self.location
    }
}
