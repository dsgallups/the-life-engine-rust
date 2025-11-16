use bevy::prelude::*;

#[derive(Component)]
pub struct CellInput {
    outputs: Vec<f32>,
}
impl CellInput {
    pub fn new(num_inputs: usize) -> Self {
        Self {
            outputs: vec![0.; num_inputs],
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    //todo
}
