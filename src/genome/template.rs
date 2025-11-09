use uuid::Uuid;

use crate::genome::{BrainGenome, EyeGenome, LauncherTemplate};

/// A cell will have these number of inputs and outputs.
pub struct PartialNetworkTemplate {
    input_junctions: usize,
    output_junctions: usize,
}
impl PartialNetworkTemplate {
    pub fn new(input_junctions: usize, output_junctions: usize) -> Self {
        Self {
            input_junctions,
            output_junctions,
        }
    }
    pub fn input_junctions(&self) -> usize {
        self.input_junctions
    }
    pub fn output_junctions(&self) -> usize {
        self.output_junctions
    }
}

// ///We will need to make some mapping to create a network.
// #[derive(Default)]
// pub struct OrganismGenomeTemplate {
//     templates: Vec<CellTemplate>,
// }

// impl OrganismGenomeTemplate {
//     pub fn add(&mut self, template: impl Into<CellTemplate>) {
//         self.templates.push(template.into());
//     }

//     pub fn cell_templates(&self) -> impl Iterator<Item = &CellTemplate> {
//         self.templates.iter()
//     }

//     pub fn sandbox() -> Self {
//         let mut template = OrganismGenomeTemplate::default();
//         template.add(EyeGenome::default());
//         template.add(BrainGenome::default());
//         template.add(LauncherTemplate::default());

//         template
//     }
// }
