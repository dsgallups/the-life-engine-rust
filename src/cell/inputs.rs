use bevy::prelude::*;

#[derive(Component)]
pub struct CellInput {
    inputs: Vec<f32>,
}
impl CellInput {
    pub fn new(num_inputs: usize) -> Self {
        Self {
            inputs: vec![0.; num_inputs],
        }
    }
    pub fn get(&self, index: usize) -> f32 {
        self.inputs[index]
    }
}

pub(super) fn plugin(app: &mut App) {
    //todo
}
