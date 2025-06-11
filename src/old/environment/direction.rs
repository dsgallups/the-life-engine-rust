#![allow(dead_code)]
use bevy::math::Vec2;
use rand::{rngs::ThreadRng, Rng as _};

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Dir {
    pub fn delta(&self) -> Vec2 {
        match self {
            Dir::Up => Vec2::new(0., 1.),
            Dir::Down => Vec2::new(0., -1.),
            Dir::Left => Vec2::new(-1., 0.),
            Dir::Right => Vec2::new(1., 0.),
        }
    }
    pub fn reverse(&mut self) {
        match self {
            Dir::Up => *self = Dir::Down,
            Dir::Down => *self = Dir::Up,
            Dir::Left => *self = Dir::Left,
            Dir::Right => *self = Dir::Right,
        }
    }
    pub fn to_reversed(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Left,
            Dir::Right => Dir::Left,
        }
    }
    pub fn randomize(&mut self, rng: &mut ThreadRng) {
        match rng.gen_range(0..=3) {
            0 => *self = Dir::Down,
            1 => *self = Dir::Up,
            2 => *self = Dir::Left,
            3 => *self = Dir::Right,
            _ => unreachable!(),
        }
    }
    pub fn rand(rng: &mut ThreadRng) -> Self {
        match rng.gen_range(0..=3) {
            0 => Dir::Down,
            1 => Dir::Up,
            2 => Dir::Left,
            3 => Dir::Right,
            _ => unreachable!(),
        }
    }

    pub fn turn(&self, other: Dir) -> i8 {
        use Dir::*;
        match (self, other) {
            (Up, Left) | (Left, Down) | (Down, Right) | (Right, Up) => -1,
            (Up, Right) | (Right, Down) | (Down, Left) | (Left, Up) => 1,
            (Up, Down) | (Left, Right) | (Down, Up) | (Right, Left) => 2,
            (Up, Up) | (Down, Down) | (Left, Left) | (Right, Right) => 0,
        }
    }
}
