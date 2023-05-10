use crate::environment::WorldEnvironment;
use bevy::prelude::*;

#[derive(Component)]
pub struct Engine {
    fps: u16,
    env: WorldEnvironment,
    running: bool,
    actual_fps: u16,
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            fps: 60,
            env: WorldEnvironment::default(),
            running: false,
            actual_fps: 0,
        }
    }
}

impl Engine {}
