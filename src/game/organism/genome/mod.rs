mod plugin;
pub use plugin::*;

use bevy::prelude::*;

#[derive(Clone)]
pub struct CellGenome {
    kind: CellType,
    location: IVec2,
}
impl CellGenome {
    pub fn kind(&self) -> &CellType {
        &self.kind
    }
}

macro_rules! cellg {
    ($variant:ident at $x:expr, $y:expr) => {
        CellGenome {
            kind: CellType::$variant,
            location: IVec2::new($x, $y),
        }
    };
}

#[derive(Clone, Copy)]
pub enum CellType {
    Launcher,
    Collagen,
    Data,
}

#[derive(Default, Clone, Component)]
pub struct Genome {
    cells: Vec<CellGenome>,
}

impl Genome {
    pub fn sandbox() -> Self {
        let cells = vec![
            cellg!(Launcher at 1, 0),
            cellg!(Collagen at 1, 0),
            cellg!(Launcher at -1, 0),
        ];

        Self { cells }
    }
    pub fn cells(&self) -> impl Iterator<Item = &CellGenome> {
        self.cells.iter()
    }
}
