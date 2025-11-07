mod plugin;
use std::collections::HashMap;

use nora_neat::prelude::{MutationChances, NetworkTopology};
pub use plugin::*;

use bevy::prelude::*;
use rand::Rng;

#[derive(Clone)]
pub struct CellGenome {
    junction_id: Option<usize>,
    kind: CellType,
    location: IVec2,
}
impl CellGenome {
    pub fn kind(&self) -> &CellType {
        &self.kind
    }
    pub fn location(&self) -> IVec2 {
        self.location
    }
}

macro_rules! cellg {
    ($j:expr, $variant:ident $( ( $($args:expr),* $(,)? ) )? at $x:expr, $y:expr) => {
        CellGenome {
            junction_id: Some($j),
            kind: CellType::$variant $( ( $($args),* ) )?,
            location: IVec2::new($x, $y),
        }
    };

    ($variant:ident $( ( $($args:expr),* $(,)? ) )? at $x:expr, $y:expr) => {
        CellGenome {
            junction_id: None,
            kind: CellType::$variant $( ( $($args),* ) )?,
            location: IVec2::new($x, $y),
        }
    };
}

#[derive(Clone)]
pub enum CellType {
    Brain(NetworkTopology),
    Launcher,
    Eye,
    Collagen,
    Data,
}

#[derive(Clone, Component)]
pub struct Genome {
    cells: Vec<CellGenome>,
}

impl Genome {
    pub fn sandbox(rng: &mut impl Rng) -> Self {
        let network_topology =
            NetworkTopology::new_thoroughly_connected(4, 4, MutationChances::new(4), rng);

        let cells = vec![
            cellg!(Brain(network_topology) at 0, 0),
            cellg!(0, Launcher at 1, 0),
            cellg!(1, Launcher at 0, 1),
            cellg!(2, Eye at 0, 2),
            cellg!(Collagen at -1, 0),
            cellg!(3, Data at 0, -1),
        ];

        Self { cells }
    }
    pub fn cells(&self) -> impl Iterator<Item = &CellGenome> {
        self.cells.iter()
    }
}
