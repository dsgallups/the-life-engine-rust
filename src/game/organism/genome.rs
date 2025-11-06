use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    //todo
}

pub struct CellGenome {
    kind: CellType,
    location: IVec2,
}
macro_rules! cellg {
    ($variant:ident at $x:expr, $y:expr) => {
        CellGenome {
            kind: CellType::$variant,
            location: IVec2::new($x, $y),
        }
    };
}

pub enum CellType {
    Defender,
    Launcher,
    Data,
}

#[derive(Default)]
pub struct Genome {
    cells: Vec<CellGenome>,
}

impl Genome {
    pub fn sandbox() -> Self {
        let cells = vec![
            cellg!(Defender at 1, 0),
            cellg!(Defender at 1, 0),
            cellg!(Defender at -1, 0),
        ];

        Self { cells }
    }
}
