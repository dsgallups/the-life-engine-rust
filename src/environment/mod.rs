use crate::organism::Organism;
use primitive_types::U256;
pub trait Environment {
    fn update(&mut self);
    fn change_cell(&mut self, c: u64, r: u64, state: u64, owner: Organism);
}

pub struct WorldEnvironment {
    // /controller: EnvironmentController,
    grid_map: GridMap,
    organisms: Vec<Organism>,
    //walls: Vec<Wall>,
    total_mutability: u64,
    largest_cell_count: u64,
    reset_count: u64,
    total_ticks: U256,
    data_update_rate: u64,
}

impl Default for WorldEnvironment {
    fn default() -> Self {
        WorldEnvironment {
            grid_map: GridMap::default(),
            organisms: Vec::new(),
            total_mutability: 0,
            largest_cell_count: 0,
            reset_count: 0,
            total_ticks: U256::zero(),
            data_update_rate: 0,
        }
    }
}

pub struct OrganismEditor {
    is_active: bool,
    cell_size: u16,
    //renderer: Renderer,
    //controller: EditorController,
    grid_map: GridMap,
}

pub struct GridMap {
    num_rows: u64,
    num_cols: u64,
}

impl Default for GridMap {
    fn default() -> Self {
        GridMap {
            num_rows: 0,
            num_cols: 0,
        }
    }
}
