use bevy::prelude::*;

#[derive(Component)]
pub struct CellOutput {
    outputs: Vec<f32>,
}
impl CellOutput {
    pub fn new(num_outputs: usize) -> Self {
        Self {
            outputs: vec![0.; num_outputs],
        }
    }
    pub fn set(&mut self, index: usize, value: f32) {
        self.outputs[index] = value;
    }
}

pub(super) fn plugin(app: &mut App) {
    //todo
}
