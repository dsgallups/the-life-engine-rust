use bevy::math::IVec2;
use rand::{Rng, seq::SliceRandom};
use strum::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, Hash, PartialEq, Eq)]
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

    pub fn random_order(rng: &mut impl Rng) -> [Self; 4] {
        let mut directions = Direction::all();
        directions.shuffle(rng);
        directions
    }

    pub fn vec(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, 1),
            Direction::Down => IVec2::new(0, -1),
            Direction::Right => IVec2::new(1, 0),
            Direction::Left => IVec2::new(-1, 0),
        }
    }
}
