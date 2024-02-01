use std::ops::Add;

use bevy::{
    ecs::{bundle::Bundle, component::Component, system::Resource},
    math::I64Vec2,
};
use rustc_hash::FxHashMap;

#[derive(Debug, PartialEq, Default, Clone, Component, Copy)]
pub struct WorldLocation(I64Vec2);

impl WorldLocation {
    pub fn new(x: i64, y: i64) -> Self {
        Self(I64Vec2::new(x, y))
    }
    pub fn set_x(&mut self, x: i64) {
        self.0.x = x;
    }
    pub fn set_y(&mut self, y: i64) {
        self.0.y = y;
    }

    pub fn x(&self) -> i64 {
        self.0.x
    }

    pub fn y(&self) -> i64 {
        self.0.y
    }
}

impl<T> From<T> for WorldLocation
where
    T: Into<I64Vec2>,
{
    fn from(location: T) -> Self {
        Self(location.into())
    }
}

impl Add for WorldLocation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
