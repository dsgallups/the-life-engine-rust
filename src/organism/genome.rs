use std::ops::Add;

use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng as _};

use crate::{cell::*, neighbor::VecExt};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellLocation {
    pub x: i32,
    pub y: i32,
}

impl VecExt for CellLocation {
    fn as_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

impl Add for CellLocation {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl CellLocation {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct OrganismCell {
    cell_type: OrganismCellType,
    /// Relative location
    location: CellLocation,
}

#[derive(Bundle)]
pub struct OrganismCellBundle {
    sprite: SpriteBundle,
    local_location: CellLocation,
    cell_type: CellType,
}

impl OrganismCell {
    pub fn new(cell_type: OrganismCellType, location: CellLocation) -> Self {
        Self {
            cell_type,
            location,
        }
    }

    pub fn as_bundle(&self) -> OrganismCellBundle {
        OrganismCellBundle {
            sprite: SpriteBundle {
                transform: Transform::from_translation(self.location.as_vec3()),
                sprite: Sprite {
                    color: self.cell_type.color(),
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            local_location: self.location,
            cell_type: self.cell_type.into(),
        }
    }

    pub fn new_rand(rng: &mut ThreadRng, location: CellLocation) -> Self {
        Self {
            cell_type: OrganismCellType::new_rand(rng),
            location,
        }
    }

    pub fn mutate(&mut self, rng: &mut ThreadRng) {
        self.cell_type = OrganismCellType::new_rand(rng);
    }

    pub fn location(&self) -> CellLocation {
        self.location
    }

    pub fn spawn(&self, child_builder: &mut ChildBuilder) {
        use OrganismCellType::*;
        match self.cell_type {
            Armor => child_builder.spawn((self.as_bundle(), ArmorCell)),
            Eye => child_builder.spawn((self.as_bundle(), EyeCell)),
            Killer => child_builder.spawn((self.as_bundle(), KillerCell)),
            Mover => child_builder.spawn((self.as_bundle(), MoverCell)),
            Producer => child_builder.spawn((self.as_bundle(), ProducerCell::default())),
            Mouth => child_builder.spawn((self.as_bundle(), MouthCell)),
        };
    }
}

#[derive(Clone, Debug)]
pub struct Genome {
    cells: Vec<OrganismCell>,
}

impl Genome {
    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }
    pub fn first_organism() -> Self {
        Self {
            cells: vec![
                OrganismCell::new(OrganismCellType::Producer, CellLocation::new(-1, -1)),
                OrganismCell::new(OrganismCellType::Mouth, CellLocation::new(0, 0)),
                OrganismCell::new(OrganismCellType::Producer, CellLocation::new(1, 1)),
            ],
        }
    }

    pub fn types(&self) -> impl Iterator<Item = OrganismCellType> + '_ {
        self.cells.iter().map(|c| c.cell_type)
    }

    pub fn spawn_cells(&self, child_builder: &mut ChildBuilder) {
        for cell in self.cells.iter() {
            cell.spawn(child_builder);
        }
    }

    pub fn cells(&self) -> impl Iterator<Item = &OrganismCell> {
        self.cells.iter()
    }

    pub fn add_random_cell(&mut self, rng: &mut ThreadRng) {
        let attach_to = if self.cells.is_empty() {
            CellLocation::new(0, 0)
        } else {
            //pick a random location in the list
            self.cells
                .get(rng.gen_range(0..self.cells.len()))
                .map(|cell| cell.location)
                .unwrap()
        };

        //pick a random place to start
        let mut x = rng.gen_range(-1..=1);
        let mut y = rng.gen_range(-1..=1);
        if x == 0 && y == 0 {
            if rng.gen::<bool>() {
                x = if rng.gen::<bool>() { 1 } else { -1 };
            } else {
                y = if rng.gen::<bool>() { 1 } else { -1 };
            }
        }

        let mut count = 0;
        loop {
            if count > 11 {
                error!("This spawn couldn't add an organ after 11 randomized attempts!");
                break;
            }
            if self
                .cells
                .iter()
                .any(|c| c.location == attach_to + CellLocation::new(x, y))
            {
                if x == 1 {
                    if y == -1 {
                        y = 0
                    } else if y == 0 {
                        y = 1
                    } else if y == 1 {
                        x = 0
                    }
                } else if x == 0 {
                    if y == -1 {
                        x = 1
                    } else if y == 1 {
                        x = -1
                    }
                } else if x == -1 {
                    if y == -1 {
                        x = 0;
                    } else if y == 0 {
                        y = -1;
                    } else if y == 1 {
                        y = 0;
                    }
                }
                count += 1;
            } else {
                self.cells.push(OrganismCell::new_rand(
                    rng,
                    attach_to + CellLocation::new(x, y),
                ));
                break;
            }
        }
    }

    pub fn mutate_random_cell(&mut self, rng: &mut ThreadRng) {
        if self.cells.is_empty() {
            return;
        };
        let new_organs_len = self.cells.len();
        let organ_to_mutate = self
            .cells
            .get_mut(rng.gen_range(0..new_organs_len))
            .unwrap();
        organ_to_mutate.mutate(rng);
    }

    pub fn delete_random_cell(&mut self, rng: &mut ThreadRng) -> Option<OrganismCell> {
        if self.cells.is_empty() {
            return None;
        }
        Some(self.cells.swap_remove(rng.gen_range(0..self.cells.len())))
    }
}
