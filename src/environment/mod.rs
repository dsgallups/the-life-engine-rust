use primitive_types::U256;

pub trait Environment {
    fn update(&mut self);
    fn changeCell(&mut self, c: u64, r: u64, state: u64, owner: Organism);
}

pub struct WorldEnvironment {
    renderer: Renderer,
    controller: EnvironmentController,
    num_rows: u64,
    num_cols: u64,
    grid_map: GridMap,
    organisms: Vec<Organism>,
    walls: Vec<Wall>,
    total_mutability: u64,
    largest_cell_count: u64,
    reset_count: u64,
    total_ticks: U256,
    data_update_rate: u64,
}

pub struct OrganismEditor {
    is_active: bool,
    cell_size: u16,
    renderer: Renderer,
    controller: EditorController,
    grid_map: GridMap,
}
