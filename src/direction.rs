use bevy::{ecs::component::Component, math::I64Vec2};
use rand::Rng;

#[derive(Debug, PartialEq, Default, Copy, Clone, Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Direction {
    pub fn delta(&self) -> I64Vec2 {
        match self {
            Direction::Up => I64Vec2::new(0, 1),
            Direction::Down => I64Vec2::new(0, -1),
            Direction::Left => I64Vec2::new(-1, 0),
            Direction::Right => I64Vec2::new(1, 0),
        }
    }
    pub fn reverse(&mut self) {
        match self {
            Direction::Up => *self = Direction::Down,
            Direction::Down => *self = Direction::Up,
            Direction::Left => *self = Direction::Left,
            Direction::Right => *self = Direction::Right,
        }
    }
    pub fn to_reversed(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Left,
        }
    }
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=3) {
            0 => *self = Direction::Down,
            1 => *self = Direction::Up,
            2 => *self = Direction::Left,
            3 => *self = Direction::Right,
            _ => unreachable!(),
        }
    }
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=3) {
            0 => Direction::Down,
            1 => Direction::Up,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }

    pub fn turn(&self, other: Direction) -> i8 {
        use Direction::*;
        match (self, other) {
            (Up, Left) | (Left, Down) | (Down, Right) | (Right, Up) => -1,
            (Up, Right) | (Right, Down) | (Down, Left) | (Left, Up) => 1,
            (Up, Down) | (Left, Right) | (Down, Up) | (Right, Left) => 2,
            (Up, Up) | (Down, Down) | (Left, Left) | (Right, Right) => 0,
        }
    }
}
