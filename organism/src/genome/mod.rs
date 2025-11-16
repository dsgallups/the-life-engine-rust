pub mod activations;

mod replicator;
use replicator::*;

mod neuron;
pub use neuron::*;

mod cell_map;
pub use cell_map::*;

mod mutation;
pub use mutation::*;

pub mod mutator;

pub mod decycler;

mod direction;
pub use direction::*;

use bevy::prelude::*;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Genome {
    pub(crate) cells: CellMap,
    pub(crate) hidden: Vec<NeuronTopology<Hidden>>,
    pub(crate) mutation: MutationChances,
}
impl Genome {
    pub fn sandbox() -> Self {
        let template = [
            (CellKind::Eye, IVec2::new(0, 0)),
            (CellKind::Launcher, IVec2::new(1, 1)),
            (CellKind::Data, IVec2::new(-1, -1)),
        ];

        let mut this = Self {
            cells: CellMap::default(),
            hidden: Vec::new(),
            mutation: MutationChances::new(20),
        };

        //outputs first
        for (kind, location) in template {
            this.cells.add_cell(location, kind);
        }
        let mut hidden_nodes = Vec::new();

        for cell in this.cells.map_mut().values_mut() {
            for output in cell.outputs.iter_mut() {
                //go 1:1 between hidden and output nodes
                let hidden = NeuronTopology::hidden();
                output.add_input(&hidden);
                hidden_nodes.push(hidden);
            }
        }

        for cell in this.cells.map_mut().values_mut() {
            for hidden_node in hidden_nodes.iter_mut() {
                for input in cell.inputs.iter() {
                    hidden_node.add_input(input);
                }
            }
        }
        this.hidden = hidden_nodes;

        this
    }
    pub fn cells(&self) -> &CellMap {
        &self.cells
    }

    pub fn deep_clone(&self) -> Genome {
        let replicator = Replicator::new(self);
        replicator.replicate()
    }

    pub fn scramble(&mut self, rng: &mut impl Rng) {
        self.mutation.adjust_mutation_chances(rng);
        let mut mutation_iter = self.mutation.yield_mutations(rng);

        while let Some(action) = mutation_iter.next(rng) {
            action.perform(&mut self.cells, &mut self.hidden, rng);
        }

        Cleaner::new(self).clean();
    }

    /// Create an empty genome for testing
    #[cfg(test)]
    pub fn empty() -> Self {
        Self {
            cells: CellMap::default(),
            hidden: Vec::new(),
            mutation: MutationChances::new(50),
        }
    }

    /// Create a simple linear genome with one input cell, one hidden, and one output cell
    #[cfg(test)]
    pub fn simple_linear() -> Self {
        let mut genome = Self::empty();

        // Add an Eye cell (input)
        genome.cells.add_cell(IVec2::new(0, 0), CellKind::Eye);

        // Add a Launcher cell (output)
        genome.cells.add_cell(IVec2::new(1, 0), CellKind::Launcher);

        // Connect them through a hidden neuron
        let hidden = NeuronTopology::hidden();

        // Connect input to hidden
        if let Some(eye_cell) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
            for input in &eye_cell.inputs {
                hidden.add_input(input);
            }
        }

        // Connect hidden to output
        if let Some(launcher_cell) = genome.cells.map_mut().get_mut(&IVec2::new(1, 0)) {
            for output in &launcher_cell.outputs {
                output.add_input(&hidden);
            }
        }

        genome.hidden.push(hidden);
        genome
    }

    /// Create a genome with custom cells at specific positions
    #[cfg(test)]
    pub fn from_cells(cells: Vec<(CellKind, IVec2)>) -> Self {
        let mut genome = Self::empty();

        for (kind, location) in cells {
            genome.cells.add_cell(location, kind);
        }

        genome
    }

    /// Get the number of cells
    #[cfg(test)]
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Get the number of hidden neurons
    #[cfg(test)]
    pub fn hidden_count(&self) -> usize {
        self.hidden.len()
    }

    /// Get hidden neurons for testing
    #[cfg(test)]
    pub fn hidden_neurons(&self) -> &[NeuronTopology<Hidden>] {
        &self.hidden
    }

    /// Get mutable hidden neurons for testing
    #[cfg(test)]
    pub fn hidden_neurons_mut(&mut self) -> &mut Vec<NeuronTopology<Hidden>> {
        &mut self.hidden
    }
}

use crate::{CellKind, genome::decycler::Cleaner};

#[cfg(test)]
use {
    pretty_assertions::assert_eq,
    rand::{SeedableRng, rngs::StdRng},
};

#[test]
fn test_genome_empty_construction() {
    let genome = Genome::empty();

    assert_eq!(genome.cell_count(), 0, "Empty genome should have no cells");
    assert_eq!(
        genome.hidden_count(),
        0,
        "Empty genome should have no hidden neurons"
    );
}

#[test]
fn test_genome_sandbox_construction() {
    let genome = Genome::sandbox();

    // Sandbox should have specific cells
    assert_eq!(genome.cell_count(), 3, "Sandbox should have 3 cells");

    // Check specific cell locations
    assert!(
        genome.cells().get(&IVec2::new(0, 0)).is_some(),
        "Should have cell at (0,0)"
    );
    assert!(
        genome.cells().get(&IVec2::new(1, 1)).is_some(),
        "Should have cell at (1,1)"
    );
    assert!(
        genome.cells().get(&IVec2::new(-1, -1)).is_some(),
        "Should have cell at (-1,-1)"
    );

    // Check cell types
    assert_eq!(
        genome.cells().get(&IVec2::new(0, 0)).unwrap().kind,
        CellKind::Eye
    );
    assert_eq!(
        genome.cells().get(&IVec2::new(1, 1)).unwrap().kind,
        CellKind::Launcher
    );
    assert_eq!(
        genome.cells().get(&IVec2::new(-1, -1)).unwrap().kind,
        CellKind::Data
    );

    // Should have hidden neurons (one per output)
    assert!(
        genome.hidden_count() > 0,
        "Sandbox should have hidden neurons"
    );
}

#[test]
fn test_genome_simple_linear_construction() {
    let genome = Genome::simple_linear();

    assert_eq!(genome.cell_count(), 2, "Simple linear should have 2 cells");
    assert_eq!(
        genome.hidden_count(),
        1,
        "Simple linear should have 1 hidden neuron"
    );

    // Check cell locations
    assert!(
        genome.cells().get(&IVec2::new(0, 0)).is_some(),
        "Should have Eye at (0,0)"
    );
    assert!(
        genome.cells().get(&IVec2::new(1, 0)).is_some(),
        "Should have Launcher at (1,0)"
    );

    // Check cell types
    assert_eq!(
        genome.cells().get(&IVec2::new(0, 0)).unwrap().kind,
        CellKind::Eye
    );
    assert_eq!(
        genome.cells().get(&IVec2::new(1, 0)).unwrap().kind,
        CellKind::Launcher
    );
}

#[test]
fn test_genome_from_cells_construction() {
    let cells = vec![
        (CellKind::Eye, IVec2::new(5, 5)),
        (CellKind::Launcher, IVec2::new(-3, 2)),
        (CellKind::Data, IVec2::new(0, 0)),
        (CellKind::Collagen, IVec2::new(10, -10)),
    ];

    let genome = Genome::from_cells(cells);

    assert_eq!(genome.cell_count(), 4, "Should have 4 cells");
    assert_eq!(
        genome.hidden_count(),
        0,
        "Should have no hidden neurons initially"
    );

    // Check all cells are at correct positions
    assert_eq!(
        genome.cells().get(&IVec2::new(5, 5)).unwrap().kind,
        CellKind::Eye
    );
    assert_eq!(
        genome.cells().get(&IVec2::new(-3, 2)).unwrap().kind,
        CellKind::Launcher
    );
    assert_eq!(
        genome.cells().get(&IVec2::new(0, 0)).unwrap().kind,
        CellKind::Data
    );
    assert_eq!(
        genome.cells().get(&IVec2::new(10, -10)).unwrap().kind,
        CellKind::Collagen
    );
}

#[test]
fn test_cell_map_find_free_spot() {
    let mut genome = Genome::empty();
    let mut rng = StdRng::seed_from_u64(42);

    // Add a cell at origin
    genome.cells.add_cell(IVec2::ZERO, CellKind::Eye);

    // Find free spot should not return origin
    let free_spot = genome.cells.find_free_spot(&mut rng);
    assert_ne!(
        free_spot,
        IVec2::ZERO,
        "Should find a spot different from origin"
    );

    // The free spot should be adjacent to origin
    let distance = (free_spot - IVec2::ZERO).abs();
    assert!(
        distance.x <= 1 && distance.y <= 1,
        "Free spot should be adjacent to existing cell"
    );
}

#[test]
fn test_cell_map_find_free_spot_multiple() {
    let mut genome = Genome::empty();
    let mut rng = StdRng::seed_from_u64(42);

    // Fill a 3x3 grid except center
    for x in -1..=1 {
        for y in -1..=1 {
            if x != 0 || y != 0 {
                genome.cells.add_cell(IVec2::new(x, y), CellKind::Collagen);
            }
        }
    }

    // Find free spot should return center
    let free_spot = genome.cells.find_free_spot(&mut rng);
    assert_eq!(
        free_spot,
        IVec2::ZERO,
        "Should find the center as free spot"
    );
}

#[test]
fn test_cell_map_num_inputs_outputs() {
    let genome = Genome::from_cells(vec![
        (CellKind::Eye, IVec2::new(0, 0)),      // 2 inputs, 0 outputs
        (CellKind::Launcher, IVec2::new(1, 0)), // 0 inputs, 3 outputs
        (CellKind::Data, IVec2::new(2, 0)),     // 4 inputs, 4 outputs
        (CellKind::Collagen, IVec2::new(3, 0)), // 0 inputs, 0 outputs
    ]);

    let (num_inputs, num_outputs) = genome.cells.num_inputs_outputs();

    // 2 + 0 + 4 + 0
    assert_eq!(num_inputs, 6, "Should count all inputs");
    // 3 + 0 + 4 + 0
    assert_eq!(num_outputs, 7, "Should count all outputs");
}

#[test]
fn test_cell_map_add_and_remove() {
    let mut genome = Genome::empty();

    // Add cell
    let replaced = genome.cells.add_cell(IVec2::new(5, 5), CellKind::Eye);
    assert!(
        replaced.is_none(),
        "Should return None when adding to empty location"
    );
    assert_eq!(genome.cell_count(), 1);

    // Replace cell at same location
    let replaced = genome.cells.add_cell(IVec2::new(5, 5), CellKind::Launcher);
    assert!(
        replaced.is_some(),
        "Should return previous cell when replacing"
    );
    assert_eq!(
        replaced.unwrap().kind,
        CellKind::Eye,
        "Should return the Eye cell"
    );
    assert_eq!(genome.cell_count(), 1, "Count should still be 1");

    // Remove cell
    let removed = genome.cells.remove(&IVec2::new(5, 5));
    assert!(removed.is_some(), "Should return removed cell");
    assert_eq!(removed.unwrap().kind, CellKind::Launcher);
    assert_eq!(genome.cell_count(), 0, "Should be empty after removal");

    // Remove from empty location
    let removed = genome.cells.remove(&IVec2::new(5, 5));
    assert!(
        removed.is_none(),
        "Should return None when removing from empty location"
    );
}

#[test]
fn test_cell_requirements() {
    // Test each cell type has correct requirements
    let eye_req = CellKind::Eye.requirements();
    assert_eq!(eye_req.num_inputs, 2);
    assert_eq!(eye_req.num_outputs, 0);

    let launcher_req = CellKind::Launcher.requirements();
    assert_eq!(launcher_req.num_inputs, 0);
    assert_eq!(launcher_req.num_outputs, 3);

    let collagen_req = CellKind::Collagen.requirements();
    assert_eq!(collagen_req.num_inputs, 0);
    assert_eq!(collagen_req.num_outputs, 0);

    let data_req = CellKind::Data.requirements();
    assert_eq!(data_req.num_inputs, 4);
    assert_eq!(data_req.num_outputs, 4);
}

#[test]
fn test_cell_creation_respects_requirements() {
    let mut genome = Genome::empty();

    // Add each type of cell
    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Eye);
    genome.cells.add_cell(IVec2::new(1, 0), CellKind::Launcher);
    genome.cells.add_cell(IVec2::new(2, 0), CellKind::Data);
    genome.cells.add_cell(IVec2::new(3, 0), CellKind::Collagen);

    // Check Eye
    let eye = genome.cells.get(&IVec2::new(0, 0)).unwrap();
    assert_eq!(eye.inputs.len(), 2, "Eye should have 2 inputs");
    assert_eq!(eye.outputs.len(), 0, "Eye should have 0 outputs");

    // Check Launcher
    let launcher = genome.cells.get(&IVec2::new(1, 0)).unwrap();
    assert_eq!(launcher.inputs.len(), 0, "Launcher should have 0 inputs");
    assert_eq!(launcher.outputs.len(), 3, "Launcher should have 3 outputs");

    // Check Data
    let data = genome.cells.get(&IVec2::new(2, 0)).unwrap();
    assert_eq!(data.inputs.len(), 4, "Data should have 4 inputs");
    assert_eq!(data.outputs.len(), 4, "Data should have 4 outputs");

    // Check Collagen
    let collagen = genome.cells.get(&IVec2::new(3, 0)).unwrap();
    assert_eq!(collagen.inputs.len(), 0, "Collagen should have 0 inputs");
    assert_eq!(collagen.outputs.len(), 0, "Collagen should have 0 outputs");
}

#[test]
fn test_genome_hidden_neuron_management() {
    let mut genome = Genome::empty();

    // Add hidden neurons
    let h1 = NeuronTopology::hidden();
    let h2 = NeuronTopology::hidden();
    let h3 = NeuronTopology::hidden();

    genome.hidden_neurons_mut().push(h1);
    genome.hidden_neurons_mut().push(h2);
    genome.hidden_neurons_mut().push(h3);

    assert_eq!(genome.hidden_count(), 3, "Should have 3 hidden neurons");

    // Remove one
    genome.hidden_neurons_mut().swap_remove(1);
    assert_eq!(
        genome.hidden_count(),
        2,
        "Should have 2 hidden neurons after removal"
    );
}

#[test]
fn test_direction_vectors() {
    assert_eq!(Direction::Up.vec(), IVec2::new(0, 1));
    assert_eq!(Direction::Down.vec(), IVec2::new(0, -1));
    assert_eq!(Direction::Right.vec(), IVec2::new(1, 0));
    assert_eq!(Direction::Left.vec(), IVec2::new(-1, 0));
}

#[test]
fn test_direction_all() {
    let all_directions = Direction::all();
    assert_eq!(all_directions.len(), 4, "Should have 4 directions");

    // Check all directions are present
    assert!(all_directions.contains(&Direction::Up));
    assert!(all_directions.contains(&Direction::Down));
    assert!(all_directions.contains(&Direction::Left));
    assert!(all_directions.contains(&Direction::Right));
}

#[test]
fn test_direction_random_order() {
    let mut rng = StdRng::seed_from_u64(42);

    let order1 = Direction::random_order(&mut rng);
    let order2 = Direction::random_order(&mut rng);

    // Both should have all 4 directions
    assert_eq!(order1.len(), 4);
    assert_eq!(order2.len(), 4);

    // Check all directions are present in both
    for dir in Direction::all() {
        assert!(order1.contains(&dir));
        assert!(order2.contains(&dir));
    }

    // Orders might be different (depending on RNG)
    // This is probabilistic but should usually pass
}

#[test]
fn test_genome_with_interconnected_cells() {
    let mut genome = Genome::empty();

    // Create a small network
    genome.cells.add_cell(IVec2::new(0, 0), CellKind::Eye);
    genome.cells.add_cell(IVec2::new(1, 0), CellKind::Data);
    genome.cells.add_cell(IVec2::new(2, 0), CellKind::Launcher);

    // Add hidden neurons and create connections
    let h1 = NeuronTopology::hidden();
    let h2 = NeuronTopology::hidden();

    // Connect Eye inputs to h1
    if let Some(eye) = genome.cells.map_mut().get_mut(&IVec2::new(0, 0)) {
        for input in &eye.inputs {
            h1.add_input(input);
        }
    }

    // Connect h1 to h2
    h2.add_input(&h1);

    // Connect Data cell
    if let Some(data) = genome.cells.map_mut().get_mut(&IVec2::new(1, 0)) {
        // Data inputs to h1
        for input in &data.inputs {
            h1.add_input(input);
        }
        // h2 to Data outputs
        for output in &data.outputs {
            output.add_input(&h2);
        }
    }

    // Connect h2 to Launcher outputs
    if let Some(launcher) = genome.cells.map_mut().get_mut(&IVec2::new(2, 0)) {
        for output in &launcher.outputs {
            output.add_input(&h2);
        }
    }

    genome.hidden_neurons_mut().push(h1);
    genome.hidden_neurons_mut().push(h2);

    assert_eq!(genome.cell_count(), 3, "Should have 3 cells");
    assert_eq!(genome.hidden_count(), 2, "Should have 2 hidden neurons");
}

#[test]
fn test_cell_map_capacity() {
    // Test with_capacity constructor
    let cell_map = CellMap::with_capacity(100);
    assert!(cell_map.is_empty(), "Should start empty");
    assert_eq!(cell_map.len(), 0, "Should have length 0");
}

#[test]
fn test_large_genome_construction() {
    let mut cells = Vec::new();

    // Create a 10x10 grid of cells
    for x in -5..5_i32 {
        for y in -5..5_i32 {
            let kind = match (x + y).abs() % 4 {
                0 => CellKind::Eye,
                1 => CellKind::Launcher,
                2 => CellKind::Data,
                _ => CellKind::Collagen,
            };
            cells.push((kind, IVec2::new(x, y)));
        }
    }

    let genome = Genome::from_cells(cells);

    assert_eq!(genome.cell_count(), 100, "Should have 100 cells");

    // Verify all positions are filled
    for x in -5..5 {
        for y in -5..5 {
            assert!(
                genome.cells().get(&IVec2::new(x, y)).is_some(),
                "Should have cell at ({}, {})",
                x,
                y
            );
        }
    }
}

#[test]
fn test_genome_mutation_chances() {
    let genome = Genome::empty();
    let mut rng = StdRng::seed_from_u64(42);

    // Should be able to yield mutations
    let mut iter = genome.mutation.yield_mutations(&mut rng);

    // Collect some mutations
    let mut mutation_count = 0;
    while let Some(_action) = iter.next(&mut rng) {
        mutation_count += 1;
        if mutation_count > 10 {
            break; // Prevent infinite loop
        }
    }

    // Should have yielded at least some mutations (depending on chances)
    assert!(mutation_count > 0, "Should yield at least some mutations");
}

#[test]
fn test_genome_sandbox_has_connections() {
    let genome = Genome::sandbox();

    // Sandbox genome should have connected neurons
    // Check that outputs have inputs (hidden neurons)
    for cell in genome.cells().map().values() {
        for output in &cell.outputs {
            output.with_ref(|neuron| {
                assert!(
                    !neuron.inputs().is_empty(),
                    "Sandbox outputs should have connections"
                );
            });
        }
    }

    // Check that hidden neurons have inputs
    for hidden in genome.hidden_neurons() {
        hidden.with_ref(|neuron| {
            assert!(
                !neuron.inputs().is_empty(),
                "Sandbox hidden neurons should have inputs"
            );
        });
    }
}

#[test]
fn test_cell_at_negative_coordinates() {
    let mut genome = Genome::empty();

    // Test cells at negative coordinates work properly
    genome.cells.add_cell(IVec2::new(-100, -100), CellKind::Eye);
    genome
        .cells
        .add_cell(IVec2::new(-50, 50), CellKind::Launcher);
    genome.cells.add_cell(IVec2::new(75, -75), CellKind::Data);

    assert_eq!(genome.cell_count(), 3);
    assert!(genome.cells.get(&IVec2::new(-100, -100)).is_some());
    assert!(genome.cells.get(&IVec2::new(-50, 50)).is_some());
    assert!(genome.cells.get(&IVec2::new(75, -75)).is_some());
}

#[test]
fn test_genome_accessors() {
    let genome = Genome::sandbox();

    // Test read-only accessors
    let cells = genome.cells();
    assert!(!cells.is_empty(), "Should have cells");

    let hidden = genome.hidden_neurons();
    assert!(!hidden.is_empty(), "Should have hidden neurons");
}
