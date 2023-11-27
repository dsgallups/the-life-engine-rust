#[derive(PartialEq)]
pub enum CellType {
    Armor,
    Body,
    Eye,
    Killer,
    Mouth,
    Mover,
    Producer,
    Food,
    Empty,
    Wall,
}

pub struct Cell {
    pub cell_type: CellType,
    pub local_x: i64,
    pub local_y: i64,
}
