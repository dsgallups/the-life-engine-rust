use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng as _};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganismCellType {
    Armor,
    Eye,
    Killer,
    Mover,
    Producer,
    Mouth,
}

impl OrganismCellType {
    pub fn color(&self) -> Color {
        use OrganismCellType::*;
        match self {
            Armor => Color::linear_rgb(0.5, 0.0, 0.5),
            Producer => Color::linear_rgb(0., 1., 0.),
            Mouth => Color::linear_rgb(1.0, 0.65, 0.),
            Mover => Color::linear_rgb(0.49, 1.0, 0.83),
            Killer => Color::linear_rgb(1.0, 0.0, 0.0),
            Eye => Color::linear_rgb(0.98, 0.5, 0.45),
        }
    }

    pub fn new_rand(rng: &mut ThreadRng) -> Self {
        use OrganismCellType::*;

        match rng.gen_range(0..=5) {
            0 => Armor,
            1 => Eye,
            2 => Killer,
            3 => Mover,
            4 => Producer,
            5 => Mouth,
            _ => unreachable!(),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum EnvironmentCellType {
    Food,
    Wall,
}

impl EnvironmentCellType {
    pub fn color(&self) -> Color {
        use EnvironmentCellType::*;
        match self {
            Food => Color::linear_rgb(0., 0., 1.0),
            Wall => Color::linear_rgb(0.2, 0.2, 0.2),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellType {
    Organism(OrganismCellType),
    Environment(EnvironmentCellType),
}

impl CellType {
    #[allow(dead_code)]
    pub fn color(&self) -> Color {
        use CellType::*;
        match self {
            Organism(o) => o.color(),
            Environment(e) => e.color(),
        }
    }
    pub fn food() -> Self {
        Self::Environment(EnvironmentCellType::Food)
    }
}

impl From<EnvironmentCellType> for CellType {
    fn from(value: EnvironmentCellType) -> Self {
        CellType::Environment(value)
    }
}

impl From<OrganismCellType> for CellType {
    fn from(value: OrganismCellType) -> Self {
        CellType::Organism(value)
    }
}
