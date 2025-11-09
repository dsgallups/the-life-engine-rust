use uuid::Uuid;

use crate::network::{BrainTemplate, EyeTemplate, LauncherTemplate};

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

pub trait CellTemplate {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }
    fn template(&self) -> PartialNetworkTemplate;
}

///We will need to make some mapping to create a network.
#[derive(Default)]
pub struct OrganismGenomeTemplate {
    templates: Vec<Box<dyn CellTemplate>>,
}

impl OrganismGenomeTemplate {
    pub fn add(&mut self, template: impl CellTemplate + 'static) {
        self.templates.push(Box::new(template));
    }

    pub fn cell_templates(&self) -> impl Iterator<Item = &dyn CellTemplate> {
        self.templates.iter().map(|template| template.as_ref())
    }

    pub fn sandbox() -> Self {
        let mut template = OrganismGenomeTemplate::default();
        template.add(EyeTemplate::default());
        template.add(BrainTemplate::default());
        template.add(LauncherTemplate::default());

        template
    }
}
