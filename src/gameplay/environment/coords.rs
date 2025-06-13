use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::gameplay::{GameState, environment::GridSet};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PreUpdate,
        set_initial_frame_coords.run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        Update,
        sync_transform_with_coords.in_set(GridSet::SyncTransforms),
    );
}

pub const DIRECTIONS: [Direction; 4] = Direction::all();

#[derive(Component, PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[require(Transform)]
#[require(InitialFrameCoords)]
pub struct GlobalCoords(pub IVec2);

impl GlobalCoords {
    pub(super) fn as_translation(&self) -> Vec3 {
        Vec3::new(self.0.x as f32, self.0.y as f32, 0.)
    }
    pub fn directional_move(&self, direction: Direction) -> GlobalCoords {
        match direction {
            Direction::Up => Self(self.0 + IVec2::Y),
            Direction::Down => Self(self.0 - IVec2::Y),
            Direction::Left => Self(self.0 - IVec2::X),
            Direction::Right => Self(self.0 + IVec2::X),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    pub fn random_order() -> [Self; 4] {
        let mut rng = rand::rng();
        let mut directions = Direction::all();
        directions.shuffle(&mut rng);
        directions
    }
}

#[derive(Component, Deref, DerefMut, Default)]
struct InitialFrameCoords(IVec2);

fn set_initial_frame_coords(mut coords: Query<(&GlobalCoords, &mut InitialFrameCoords)>) {
    for (cur, mut prev) in &mut coords {
        prev.0 = cur.0

        //todo
    }
}

fn sync_transform_with_coords(
    mut coords: Query<(&mut Transform, &GlobalCoords), Changed<GlobalCoords>>,
) {
    for (mut transform, coords) in &mut coords {
        *transform = transform.with_translation(coords.as_translation());
        //todo
    }
}
