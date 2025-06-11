use bevy::prelude::*;

use crate::gameplay::GameSet;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<GameTick>()
        .init_resource::<GameTick>()
        .add_systems(Update, add_tick.in_set(GameSet::TickTimers));
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct GameTick {
    tick: usize,
}

impl GameTick {
    pub fn current_tick(&self) -> usize {
        self.tick
    }
}

fn add_tick(mut game_ticks: ResMut<GameTick>) {
    game_ticks.tick += 1;
}
