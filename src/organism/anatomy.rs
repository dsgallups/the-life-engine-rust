use crate::organism::cell::Cell;

use super::cell::CellType;

pub struct Anatomy {
    cells: Vec<Cell>,
    anatomy_function: AnatomyFunction,
    has_eye: bool,
    birth_distance: u64,
}

impl Default for Anatomy {
    fn default() -> Self {
        Anatomy {
            cells: Vec::new(),
            anatomy_function: AnatomyFunction::None,
            has_eye: false,
            birth_distance: 0,
        }
    }
}

impl Anatomy {
    pub fn new(cells: Vec<Cell>) -> Self {
        let mut anatomy_function = AnatomyFunction::Mover;
        let mut has_eye = false;
        for cell in &cells {
            if cell.cell_type == CellType::Producer {
                anatomy_function = AnatomyFunction::Producer;
            }
            if cell.cell_type == CellType::Eye {
                has_eye = true;
            }
        }

        Anatomy {
            cells,
            anatomy_function,
            has_eye,
            birth_distance: 0,
        }
    }
}

pub enum AnatomyFunction {
    Mover,
    Producer,
    None,
}
