use crate::organism::cell::Cell;

pub struct Anatomy {
    cells: Vec<Cell>,
    anatomy_function: AnatomyFunction,
    has_eyes: bool,
    birth_distance: u64,
}

impl Default for Anatomy {
    fn default() -> Self {
        Anatomy {
            cells: Vec::new(),
            anatomy_function: AnatomyFunction::None,
            has_eyes: false,
            birth_distance: 0,
        }
    }
}

pub enum AnatomyFunction {
    Mover,
    Producer,
    None,
}
