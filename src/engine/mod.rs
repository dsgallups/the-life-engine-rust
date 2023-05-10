use crate::environment::WorldEnvironment;
use bevy::prelude::*;

#[derive(Component)]
pub struct Engine<'a> {
    fps: u16,
    env: WorldEnvironment<'a>,
    running: bool,
    actual_fps: u16,
}

impl Default for Engine<'_> {
    fn default() -> Self {
        Engine {
            fps: 60,
            env: WorldEnvironment::default(),
            running: false,
            actual_fps: 0,
        }
    }
}

impl Engine<'_> {}
