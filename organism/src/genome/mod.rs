mod plugin;
use nora_neat::prelude::NetworkTopology;
pub use plugin::*;

mod template;
pub use template::*;

mod builder;
pub use builder::*;

use bevy::prelude::*;
use rand::Rng;

use crate::{CellGenome, CellTemplate};

#[derive(Clone, Component)]
pub struct Genome {
    cells: Vec<CellGenome>,
    network_topology: NetworkTopology,
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
