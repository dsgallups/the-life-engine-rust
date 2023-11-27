use crate::life_engine::{Drawable, OrganismCell};
use bevy::math::I64Vec3;
use bevy::math::U64Vec3;
use bevy::prelude::*;
use bevy::utils::Uuid;
use std::fmt::Debug;

#[derive(Clone, Debug, Default)]
pub struct Organ {
    cell: OrganismCell,
    relative_location: I64Vec3,
}

impl Organ {
    pub fn new(cell: OrganismCell, relative_location: I64Vec3) -> Organ {
        Organ {
            cell,
            relative_location,
        }
    }
    pub fn position(&self) -> &I64Vec3 {
        &self.relative_location
    }
    pub fn color(&self) -> Color {
        self.cell.color()
    }
    pub fn cell(&self) -> OrganismCell {
        self.cell.clone()
    }
}

#[derive(Default, Component)]
pub struct Organism {
    id: Uuid,
    organs: Vec<Organ>,
    location: U64Vec3,
    food_collected: u64,
}

impl Organism {
    pub fn new(organs: Vec<Organ>, location: U64Vec3) -> Organism {
        Organism {
            id: Uuid::new_v4(),
            organs,
            location,
            food_collected: 0,
        }
    }

    pub fn update_request(&self) -> UpdateRequest {
        UpdateRequest {
            place_food: Vec::new(),
        }
    }

    pub fn tick_response(&mut self, _response: TickResponse) {}

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn origin(&self) -> &U64Vec3 {
        &self.location
    }

    pub fn organs(&self) -> &[Organ] {
        &self.organs
    }
}

pub struct UpdateRequest {
    place_food: Vec<I64Vec3>,
}

pub struct TickResponse {}
