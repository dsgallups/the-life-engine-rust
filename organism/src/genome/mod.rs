mod plugin;
pub use plugin::*;

mod template;
pub use template::*;

mod cells;
pub use cells::*;

mod builder;
pub use builder::*;

use nora_neat::{
    neuron::{NeuronInput, NeuronTopology, Topology},
    prelude::{MutationChances, NetworkTopology},
};

use bevy::{platform::collections::HashMap, prelude::*};
use rand::Rng;
use uuid::Uuid;

#[derive(Clone)]
pub struct CellGenome {
    id: Uuid,
    kind: CellDetails,
    location: IVec2,
}
impl CellGenome {
    pub fn details(&self) -> &CellDetails {
        &self.kind
    }
    pub fn location(&self) -> IVec2 {
        self.location
    }
}

// macro_rules! cellg {
//     ($j:expr, $variant:ident $( ( $($args:expr),* $(,)? ) )? at $x:expr, $y:expr) => {
//         CellGenome {
//             junction_id: Some($j),
//             kind: CellDetails::$variant $( ( $($args),* ) )?,
//             location: IVec2::new($x, $y),
//         }
//     };

//     ($variant:ident $( ( $($args:expr),* $(,)? ) )? at $x:expr, $y:expr) => {
//         CellGenome {
//             junction_id: None,
//             kind: CellDetails::$variant $( ( $($args),* ) )?,
//             location: IVec2::new($x, $y),
//         }
//     };
// }

#[derive(Clone)]
pub enum CellDetails {
    Brain(NetworkTopology),
    Launcher,
    Eye,
    Collagen,
    Data,
}
impl CellDetails {
    pub fn cell_type(&self) -> CellKind {
        match self {
            CellDetails::Brain(_) => CellKind::Brain,
            CellDetails::Collagen => CellKind::Collagen,
            CellDetails::Data => CellKind::Data,
            CellDetails::Eye => CellKind::Eye,
            CellDetails::Launcher => CellKind::Launcher,
        }
    }
}

#[derive(Component, Reflect, Clone, Copy, PartialEq, Eq)]
pub enum CellKind {
    Brain,
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
    pub fn new(cell_templates: Vec<CellTemplate>, rng: &mut impl Rng) -> Self {
        let builder = GenomeBuilder::new(cell_templates);
        builder.build(rng)
    }
    pub fn cells(&self) -> impl Iterator<Item = &CellGenome> {
        self.cells.iter()
    }
}
