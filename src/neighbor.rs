#![allow(clippy::wrong_self_convention)]
#![allow(dead_code)]
use bevy::prelude::*;
use bevy_spatial::SpatialAccess;
use rand::{seq::SliceRandom as _, thread_rng, Rng as _};
use std::ops::Deref;

use crate::{CellTree, ORGANISM_LAYER};
pub trait VecExt: Sized {
    fn as_vec3(self) -> Vec3;

    fn as_vec2(self) -> Vec2;

    /// Returns a location for placing food if there is one, exactly one north, east, west, or south
    fn get_free_space(self, tree: &impl KDTreeExt) -> Option<Vec2> {
        self.as_vec2().get_free_space(tree)
    }

    /// Returns an iterator of surrounding entities
    fn get_surrounding_entities(self, tree: &impl KDTreeExt) -> impl Iterator<Item = Entity> {
        self.as_vec2().get_surrounding_entities(tree)
    }

    fn around(self) -> [Vec2; 4] {
        self.as_vec2().around()
    }

    fn around_nosfht(self) -> [Vec2; 4] {
        self.as_vec2().around_nosfht()
    }

    fn add_x(self, add: f32) -> Vec2 {
        self.as_vec2().add_x(add)
    }

    fn add_y(self, add: f32) -> Vec2 {
        self.as_vec2().add_y(add)
    }

    /// returns a random location in ths radius
    fn rand_in(radius: u16) -> Vec2 {
        Vec2::rand_in(radius)
    }

    /// returns a random location within the radius (translated) (actually it's a square rn cuz im lazy)
    fn rand_around(self, around: u16) -> Vec2 {
        self.as_vec2().rand_around(around)
    }
}

impl VecExt for Vec2 {
    fn as_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, ORGANISM_LAYER)
    }
    fn as_vec2(self) -> Vec2 {
        self
    }

    fn get_surrounding_entities(self, tree: &impl KDTreeExt) -> impl Iterator<Item = Entity> {
        let around = self.around_nosfht();

        tree.closest_neighbors(self)
            .filter_map(move |(loc, entity)| {
                if entity.is_none() || !around.contains(&loc) {
                    None
                } else {
                    entity
                }
            })
    }

    fn get_free_space(self, tree: &impl KDTreeExt) -> Option<Vec2> {
        let closest_neighbors = tree.closest_neighbors(self);
        let around = self.around();

        // we only get to iterate once through closest_neighbors, so neighbors has to be iterated on the outside,
        // and a dance has to occur.
        let mut valid = [true; 4];
        for (neighbor_loc, _) in closest_neighbors {
            for (potential_loc, valid) in around.iter().zip(valid.iter_mut()) {
                if !*valid {
                    continue;
                }
                if &neighbor_loc == potential_loc {
                    *valid = false;
                }
            }
        }

        for (i, is_valid) in valid.into_iter().enumerate() {
            if is_valid {
                return Some(*around.get(i).unwrap());
            }
        }

        None
    }

    fn add_x(self, add: f32) -> Vec2 {
        Vec2 {
            x: (self.x + add).round(),
            y: self.y,
        }
    }

    fn add_y(self, add: f32) -> Vec2 {
        Vec2 {
            x: self.x,
            y: (self.y + add).round(),
        }
    }

    fn rand_in(radius: u16) -> Vec2 {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(-(radius as i32)..=(radius as i32));
        let y = rng.gen_range(-(radius as i32)..=(radius as i32));

        Self {
            x: x as f32,
            y: y as f32,
        }
    }

    fn rand_around(self, radius: u16) -> Vec2 {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(-(radius as i32)..=(radius as i32));
        let y = rng.gen_range(-(radius as i32)..=(radius as i32));

        Self {
            x: (self.x + x as f32).round(),
            y: (self.y + y as f32).round(),
        }
    }

    fn around(self) -> [Vec2; 4] {
        let vec = self.round();
        let mut arr = [vec.add_x(1.), vec.add_y(1.), vec.add_x(-1.), vec.add_y(-1.)];
        let mut rng = thread_rng();
        arr.shuffle(&mut rng);
        arr
    }
    fn around_nosfht(self) -> [Vec2; 4] {
        let vec = self.round();
        let mut arr = [vec.add_x(1.), vec.add_y(1.), vec.add_x(-1.), vec.add_y(-1.)];
        let mut rng = thread_rng();
        arr.shuffle(&mut rng);
        arr
    }
}

impl VecExt for Vec3 {
    fn as_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    fn as_vec3(self) -> Vec3 {
        self
    }
}

pub trait KDTreeExt {
    fn closest_neighbors(
        &self,
        global_transform: impl VecExt,
    ) -> impl Iterator<Item = (Vec2, Option<Entity>)>;
}

impl<T> KDTreeExt for T
where
    T: Deref<Target = CellTree>,
{
    fn closest_neighbors(
        &self,
        global_transform: impl VecExt,
    ) -> impl Iterator<Item = (Vec2, Option<Entity>)> {
        self.k_nearest_neighbour(global_transform.as_vec2(), 5)
            .into_iter()
            .skip(1)
    }
}
