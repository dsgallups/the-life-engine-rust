use crate::organism::cell::Cell;

use super::{cell::CellType, direction::Direction};

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
            anatomy_function: AnatomyFunction::mover(),
            has_eye: false,
            birth_distance: 0,
        }
    }
}

impl Anatomy {
    pub fn new(cells: Vec<Cell>) -> Self {
        let mut anatomy_function = AnatomyFunction::producer();
        let mut has_eye = false;
        for cell in &cells {
            if cell.cell_type == CellType::Mover {
                anatomy_function = AnatomyFunction::mover();
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
    Mover(MoverProps),
    Producer(ProducerProps),
    None,
}

impl AnatomyFunction {
    pub fn mover() -> Self {
        Self::Mover(MoverProps::default())
    }
    pub fn producer() -> Self {
        Self::Producer(ProducerProps::default())
    }
    pub fn none() -> Self {
        Self::None
    }
}

#[derive(Default)]
pub struct ProducerProps {
    pub food_produced: u64,
}

#[derive(Default)]
pub struct MoverProps {
    pub direction: Direction,
    pub rotation: Direction,
    pub move_count: u64,
    pub move_range: u64,
    pub food_collected: u64,
}
