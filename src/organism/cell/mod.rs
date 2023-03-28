enum CellType {
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
    cell_type: CellType,
    local_row: u64,
    local_col: u64,
}
