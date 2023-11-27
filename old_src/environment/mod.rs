use crate::organism::{cell::Cell, Organism};
use bevy::prelude::Resource;
use primitive_types::U256;
pub trait Environment {
    fn update(&mut self);
    fn change_cell(&mut self, c: u64, r: u64, state: u64, owner: Organism);
}

#[derive(Resource)]
pub struct WorldEnvironment<'a> {
    // /controller: EnvironmentController,
    pub grid_map: GridMap,
    organisms: Vec<Organism<'a>>,
    //walls: Vec<Wall>,
    total_mutability: u64,
    largest_cell_count: u64,
    reset_count: u64,
    total_ticks: U256,
    data_update_rate: u64,
}

impl Default for WorldEnvironment<'_> {
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
    pub num_rows: usize,
    pub num_cols: usize,
}

impl Default for GridMap {
    fn default() -> Self {
        GridMap {
            num_rows: 64,
            num_cols: 64,
        }
    }
}
