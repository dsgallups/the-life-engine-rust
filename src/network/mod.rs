mod cells;
pub use cells::*;

mod template;
pub use template::*;

pub struct OrganismGenome {}

impl OrganismGenome {
    pub fn new(genome_template: OrganismGenomeTemplate) -> Self {
        for cell_template in genome_template.cell_templates() {
            let id = cell_template.id();
            let network_template = cell_template.template();

            //todo
        }

        todo!()
    }
}
