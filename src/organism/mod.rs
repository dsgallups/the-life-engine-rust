use crate::world::OrganismCell;
use bevy::math::I64Vec3;
use bevy::prelude::*;
use bevy::utils::Uuid;
use std::fmt::Debug;

impl OrganismCell {
    pub fn color(&self) -> Color {
        use OrganismCell::*;
        match self {
            Mouth => Color::rgb(0.5, 0.4, 0.8),
            Producer => Color::rgb(0.2, 0.7, 0.1),
        }
    }
}

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
    pub fn loc(&self) -> &I64Vec3 {
        &self.relative_location
    }
    pub fn color(&self) -> Color {
        self.cell.color()
    }
}

#[derive(Default, Component)]
pub struct Organism {
    id: Uuid,
    organs: Vec<Organ>,
    location: I64Vec3,
}

impl Organism {
    pub fn new(organs: Vec<Organ>, location: I64Vec3) -> Organism {
        Organism {
            id: Uuid::new_v4(),
            organs,
            location,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn origin(&self) -> &I64Vec3 {
        &self.location
    }

    pub fn organs(&self) -> &[Organ] {
        &self.organs
    }

    pub fn draw(&self) -> Vec<SpriteBundle> {
        let mut organ_bundles = Vec::new();

        for organ in self.organs.iter() {
            let organ_loc = (*self.origin() + *organ.loc()).as_vec3();
            let color = organ.color();

            organ_bundles.push(SpriteBundle {
                sprite: Sprite { color, ..default() },
                transform: Transform::from_translation(organ_loc),
                ..default()
            });
        }

        organ_bundles
    }
}
