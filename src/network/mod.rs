mod cells;
pub use cells::*;
use uuid::Uuid;

/// A cell will have these number of inputs and outputs.
pub struct PartialNetworkTemplate {
    input_junctions: usize,
    output_junctions: usize,
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

    pub fn sandbox() -> Self {
        let mut template = OrganismGenomeTemplate::default();
        template.add(EyeTemplate::default());
        template.add(BrainTemplate::default());
        template.add(LauncherTemplate::default());

        template
    }
}
