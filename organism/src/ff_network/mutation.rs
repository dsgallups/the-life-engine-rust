use rand::{
    Rng,
    seq::{IndexedMutRandom, IteratorRandom},
};
use strum::{EnumCount, EnumIter, IntoEnumIterator};

use crate::ff_network::{
    CellKind, CellMap, Hidden, NeuronTopology,
    mutator::{ConnectionTask, Mutator, OutputTask},
};

#[derive(Copy, Clone, Debug, EnumIter, EnumCount, PartialEq, Eq)]
pub enum MutationAction {
    AddCell,
    DeleteCell,
    MutateCell,
    AddConnection,
    SplitConnection,
    RemoveNeuron,
    MutateWeight,
    MutateActivation,
}

impl MutationAction {
    pub fn perform(
        &self,
        cells: &mut CellMap,
        hidden: &mut Vec<NeuronTopology<Hidden>>,
        rng: &mut impl Rng,
    ) {
        match self {
            MutationAction::AddCell => {
                let new_cell_kind = CellKind::iter().choose(rng).unwrap();
                let new_spot = cells.find_free_spot(rng);
                cells.add_cell(new_spot, new_cell_kind);
            }
            MutationAction::DeleteCell => {
                if cells.is_empty() {
                    return;
                }
                let rand_index = rng.random_range(0..cells.len());
                let random_cell_loc = cells.map().keys().nth(rand_index).unwrap();
                let loc = *random_cell_loc;
                cells.remove(&loc);
            }
            MutationAction::MutateCell => {
                if cells.is_empty() {
                    return;
                }
                let new_cell_kind = CellKind::iter().choose(rng).unwrap();
                let rand_index = rng.random_range(0..cells.len());
                let random_cell_loc = cells.map().keys().nth(rand_index).unwrap();
                cells.add_cell(*random_cell_loc, new_cell_kind);
            }
            MutationAction::AddConnection => {
                Mutator::new(cells, hidden).with_random_input_and_output(rng, ConnectionTask::Add);
            }
            MutationAction::SplitConnection => {
                Mutator::new(cells, hidden).with_random_output(rng, OutputTask::Split);
            }
            MutationAction::RemoveNeuron => {
                if hidden.is_empty() {
                    return;
                }
                let random_index = rng.random_range(0..hidden.len());
                hidden.swap_remove(random_index);
            }
            MutationAction::MutateWeight => {
                Mutator::new(cells, hidden).with_random_output(rng, OutputTask::MutateWeight);
            }
            MutationAction::MutateActivation => {
                Mutator::new(cells, hidden).with_random_output(rng, OutputTask::MutateActivation);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MutationChance {
    action: MutationAction,
    chance: f32,
}
impl MutationChance {
    pub fn adjust_chance(&mut self, rng: &mut impl Rng) {
        // Generate a random number between 1.0 and 10.0
        let value = rng.random_range(0.0..=5.0);

        let add_to = if rng.random_bool(0.5) { -value } else { value };
        self.chance += add_to;
    }
}

pub const MAX_MUTATIONS: u8 = 200;

#[derive(Clone, Debug, PartialEq)]
pub struct MutationChances {
    self_mutation: u8,
    chances: Vec<MutationChance>,
}

impl MutationChances {
    pub fn new(self_mutation_rate: u8) -> Self {
        let len = MutationAction::COUNT;
        Self {
            self_mutation: self_mutation_rate,
            chances: MutationAction::iter()
                .map(|action| MutationChance {
                    action,
                    chance: 1. / len as f32,
                })
                .collect(),
        }
    }

    pub fn adjust_mutation_chances(&mut self, rng: &mut impl Rng) {
        const MAX_LOOP: u8 = 10;
        let mut loop_count = 0;
        while rng.random_range(0..=100) < self.self_mutation && loop_count < MAX_LOOP {
            let Some(action) = self.chances.choose_mut(rng) else {
                return;
            };

            action.adjust_chance(rng);

            loop_count += 1;
        }

        let rate: i8 = rng.random_range(-1..=1);

        if rate < 0 && self.self_mutation == 0 {
            return;
        }

        if rate > 0 && self.self_mutation == 100 {
            return;
        }

        if rate.saturating_add(self.self_mutation as i8) < 0 {
            self.self_mutation = 0;
            return;
        }

        if rate.saturating_add(self.self_mutation as i8) > 100 {
            self.self_mutation = 100;
            return;
        }

        self.self_mutation = (self.self_mutation as i8 + rate) as u8;
    }
    pub fn yield_mutations(&self, rng: &mut impl Rng) -> MutationIter<'_> {
        MutationIter::new(rng, self)
    }
}

pub struct MutationIter<'a> {
    chances: &'a MutationChances,
    total: f32,
    count: usize,
    keep_yielding: bool,
}
impl<'a> MutationIter<'a> {
    fn new(rng: &mut impl Rng, chances: &'a MutationChances) -> Self {
        let keep_yielding = rng.random_range(0..=100) > chances.self_mutation;

        let total = chances
            .chances
            .iter()
            .map(|chance| chance.chance)
            .sum::<f32>();
        Self {
            keep_yielding,
            count: 0,
            chances,
            total,
        }
    }

    pub fn next(&mut self, rng: &mut impl Rng) -> Option<MutationAction> {
        pub const MAX_MUTATIONS: usize = 200;
        if !self.keep_yielding || self.count > MAX_MUTATIONS {
            return None;
        }
        let mut chance = rng.random_range(0_f32..self.total);
        for mutation_chance in self.chances.chances.iter() {
            let mut_chance = mutation_chance.chance;
            if chance > mut_chance {
                chance -= mut_chance;
                continue;
            }
            self.keep_yielding = rng.random_range(0..=100) > self.chances.self_mutation;
            self.count += 1;
            return Some(mutation_chance.action);
        }
        None
    }
}

/*
*

pub fn self_mutation(&self) -> u8 {
    self.self_mutation
}

fn adjust_self_mutation(&mut self, rng: &mut impl Rng) {
    let rate: i8 = rng.random_range(-1..=1);

    if rate < 0 && self.self_mutation == 0 {
        return;
    }

    if rate > 0 && self.self_mutation == 100 {
        return;
    }

    if rate.saturating_add(self.self_mutation as i8) < 0 {
        self.self_mutation = 0;
        return;
    }

    if rate.saturating_add(self.self_mutation as i8) > 100 {
        self.self_mutation = 100;
        return;
    }

    self.self_mutation = (self.self_mutation as i8 + rate) as u8;
}

pub fn split_connection(&self) -> f32 {
    self.split_connection
}

pub fn add_connection(&self) -> f32 {
    self.add_connection
}

pub fn remove_connection(&self) -> f32 {
    self.remove_connection
}

pub fn mutate_weight(&self) -> f32 {
    self.mutate_weight
}

fn adjust(&mut self, cmd: impl FnOnce(&mut Self)) {
    cmd(self);
    if self.split_connection < 0. {
        self.split_connection = 0.;
    }
    if self.add_connection < 0. {
        self.add_connection = 0.;
    }
    if self.remove_connection < 0. {
        self.remove_connection = 0.;
    }
    if self.mutate_weight < 0. {
        self.mutate_weight = 0.;
    }

    self.recalculate();
}

fn adjust_split_connection(&mut self, amt: f32) {
    self.split_connection += amt;

    if self.split_connection < 0. {
        self.split_connection = 0.;
    }

    self.recalculate();
}

fn adjust_add_connection(&mut self, amt: f32) {
    self.add_connection += amt;

    if self.add_connection < 0. {
        self.add_connection = 0.;
    }

    self.recalculate();
}

fn adjust_remove_connection(&mut self, amt: f32) {
    self.remove_connection += amt;

    if self.remove_connection < 0. {
        self.remove_connection = 0.;
    }

    self.recalculate();
}

fn adjust_mutate_weight(&mut self, amt: f32) {
    self.mutate_weight += amt;

    if self.mutate_weight < 0. {
        self.mutate_weight = 0.;
    }

    self.recalculate();
}

fn recalculate(&mut self) {
    let total = self.split_connection
        + self.add_connection
        + self.remove_connection
        + self.mutate_weight;

    self.split_connection = (self.split_connection * 100.) / total;
    self.add_connection = (self.add_connection * 100.) / total;
    self.remove_connection = (self.remove_connection * 100.) / total;
    self.mutate_weight = (self.mutate_weight * 100.) / total;
}

/// Generate a sequence of mutation actions based on the configured probabilities.
///
/// This method generates multiple mutations in one go, with each subsequent
/// mutation having reduced probability. The maximum number of mutations is
/// limited by [`MAX_MUTATIONS`].
///
/// # Arguments
/// * `rng` - Random number generator for probabilistic selection
///
/// # Returns
/// A vector of mutation actions to apply to the network
pub fn gen_mutation_actions(&self, rng: &mut impl Rng) -> Vec<MutationAction> {
    let mut actions = Vec::with_capacity(MAX_MUTATIONS as usize);
    let mut replica = *self;

    let mut loop_count = 0;
    while rng.gen_rate() < replica.self_mutation() && loop_count < MAX_MUTATIONS {
        let action = rng.gen_mutation_action(&replica);
        match action {
            MutationAction::SplitConnection => replica.adjust(|s| s.split_connection /= 2.),
            MutationAction::AddConnection => replica.adjust(|s| s.add_connection /= 2.),
            MutationAction::RemoveNeuron => replica.adjust(|s| s.remove_connection /= 2.),
            MutationAction::MutateWeight => replica.adjust(|s| s.mutate_weight /= 2.),
        }

        actions.push(rng.gen_mutation_action(self));
        loop_count += 1;
    }

    actions
}


#[test]
pub fn adjust_mutation_chances() {
    let mut chances = MutationChances::new(50);

    chances.adjust_split_connection(10.);

    chances.adjust_add_connection(-10.);

    chances.adjust_remove_connection(10.);

    chances.adjust_mutate_weight(-10.);

    let total = chances.split_connection
        + chances.add_connection
        + chances.remove_connection
        + chances.mutate_weight;
    let diff = (100. - total).abs();

    assert!(diff <= 0.0001);
}

#[test]
pub fn check_mutate() {
    let mut rng = rand::rng();

    let mut chances = MutationChances::new(50);

    for _ in 0..100 {
        chances.adjust_mutation_chances(&mut rng);

        println!("{:?}", chances);

        let total = chances.split_connection
            + chances.add_connection
            + chances.remove_connection
            + chances.mutate_weight;

        let diff = (100. - total).abs();

        assert!(diff <= 0.0001);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_mutation_rate_ext_gen_rate() {
        let mut rng = StdRng::seed_from_u64(42);

        // Test that gen_rate produces values in range [0, 100]
        for _ in 0..1000 {
            let rate = rng.gen_rate();
            assert!(rate <= 100, "Rate {} should be <= 100", rate);
        }
    }

    #[test]
    fn test_mutation_rate_ext_gen_mutation_action() {
        let mut rng = StdRng::seed_from_u64(12345);
        let chances = MutationChances::new(100);

        // Generate many actions and ensure all types are represented
        let mut action_counts = std::collections::HashMap::new();
        for _ in 0..1000 {
            let action = rng.gen_mutation_action(&chances);
            *action_counts.entry(format!("{:?}", action)).or_insert(0) += 1;
        }

        // With equal chances, all actions should appear
        assert!(action_counts.contains_key("SplitConnection"));
        assert!(action_counts.contains_key("AddConnection"));
        assert!(action_counts.contains_key("RemoveNeuron"));
        assert!(action_counts.contains_key("MutateWeight"));
        assert!(action_counts.contains_key("MutateExponent"));
    }

    #[test]
    fn test_mutation_chances_new() {
        let chances = MutationChances::new(75);

        assert_eq!(chances.self_mutation(), 75);

        // All mutation types should have equal probability (20% each)
        assert!((chances.split_connection() - 20.0).abs() < 0.001);
        assert!((chances.add_connection() - 20.0).abs() < 0.001);
        assert!((chances.remove_connection() - 20.0).abs() < 0.001);
        assert!((chances.mutate_weight() - 20.0).abs() < 0.001);

        // Total should be 100%
        let total = chances.split_connection()
            + chances.add_connection()
            + chances.remove_connection()
            + chances.mutate_weight();
        assert!((total - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_mutation_chances_none() {
        let chances = MutationChances::none();

        assert_eq!(chances.self_mutation(), 0);
        assert_eq!(chances.split_connection(), 0.0);
        assert_eq!(chances.add_connection(), 0.0);
        assert_eq!(chances.remove_connection(), 0.0);
        assert_eq!(chances.mutate_weight(), 0.0);
    }

    #[test]
    fn test_mutation_chances_new_from_raw() {
        let chances = MutationChances::new_from_raw(80, 40.0, 30.0, 10.0, 15.0);

        assert_eq!(chances.self_mutation(), 80);

        // Check that values are normalized to sum to 100
        let total = chances.split_connection()
            + chances.add_connection()
            + chances.remove_connection()
            + chances.mutate_weight();
        assert!((total - 100.0).abs() < 0.001);

        // Check relative proportions
        assert!(chances.split_connection() > chances.add_connection());
        assert!(chances.add_connection() > chances.mutate_weight());
        assert!(chances.mutate_weight() > chances.remove_connection());
    }

    #[test]
    fn test_mutation_chances_new_from_raw_zero_total() {
        // Test edge case where all values are zero
        let chances = MutationChances::new_from_raw(50, 0.0, 0.0, 0.0, 0.0);

        // Should handle division by zero gracefully
        assert_eq!(chances.self_mutation(), 50);
    }

    #[test]
    fn test_adjust_self_mutation() {
        let mut rng = StdRng::seed_from_u64(42);

        // Test boundary conditions
        let mut chances = MutationChances::new(0);
        chances.adjust_self_mutation(&mut rng);
        assert!(chances.self_mutation() <= 1); // Can only go up from 0

        let mut chances = MutationChances::new(100);
        chances.adjust_self_mutation(&mut rng);
        assert!(chances.self_mutation() >= 99); // Can only go down from 100

        // Test normal adjustment
        let mut chances = MutationChances::new(50);
        let original = chances.self_mutation();
        chances.adjust_self_mutation(&mut rng);
        let diff = (chances.self_mutation() as i32 - original as i32).abs();
        assert!(diff <= 1); // Should change by at most 1
    }

    #[test]
    fn test_individual_adjustments() {
        let mut chances = MutationChances::new(50);

        // Test positive adjustments
        chances.adjust_split_connection(10.0);
        let total = chances.split_connection()
            + chances.add_connection()
            + chances.remove_connection()
            + chances.mutate_weight();
        assert!((total - 100.0).abs() < 0.001);

        // Test negative adjustments
        chances.adjust_add_connection(-5.0);
        let total = chances.split_connection()
            + chances.add_connection()
            + chances.remove_connection()
            + chances.mutate_weight();
        assert!((total - 100.0).abs() < 0.001);

        // Test adjustment below zero
        chances.adjust_remove_connection(-100.0);
        assert!(chances.remove_connection() >= 0.0);
        let total = chances.split_connection()
            + chances.add_connection()
            + chances.remove_connection()
            + chances.mutate_weight();
        assert!((total - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_gen_mutation_actions() {
        let mut rng = StdRng::seed_from_u64(9876);

        // Test with high mutation chance
        let chances = MutationChances::new(90);
        let actions = chances.gen_mutation_actions(&mut rng);
        assert!(
            !actions.is_empty(),
            "Should generate some mutations with 90% chance"
        );
        assert!(actions.len() <= MAX_MUTATIONS as usize);

        // Test with zero mutation chance
        let chances = MutationChances::none();
        let actions = chances.gen_mutation_actions(&mut rng);
        assert!(
            actions.is_empty(),
            "Should generate no mutations with 0% chance"
        );

        // Test with moderate mutation chance
        let chances = MutationChances::new(50);
        let mut total_actions = 0;
        for _ in 0..100 {
            let actions = chances.gen_mutation_actions(&mut rng);
            total_actions += actions.len();
        }
        assert!(
            total_actions > 0,
            "Should generate some mutations over 100 trials"
        );
    }

    #[test]
    fn test_gen_mutation_actions_distribution() {
        let mut rng = StdRng::seed_from_u64(5555);

        // Create chances with specific distribution
        let chances = MutationChances::new_from_raw(
            100,  // Always mutate
            50.0, // High split chance
            30.0, // Medium add chance
            5.0,  // Low remove chance
            10.0, // Low weight chance
        );

        // Collect many mutations
        let mut action_counts = std::collections::HashMap::new();
        for _ in 0..1000 {
            let actions = chances.gen_mutation_actions(&mut rng);
            for action in actions {
                *action_counts.entry(format!("{:?}", action)).or_insert(0) += 1;
            }
        }

        // Verify that split connections are most common
        let split_count = action_counts.get("SplitConnection").unwrap_or(&0);
        let add_count = action_counts.get("AddConnection").unwrap_or(&0);
        let remove_count = action_counts.get("RemoveNeuron").unwrap_or(&0);

        assert!(
            split_count > add_count,
            "Split should be more common than add"
        );
        assert!(
            add_count > remove_count,
            "Add should be more common than remove"
        );
    }

    #[test]
    fn test_deterministic_mutations() {
        let seed = 1111;
        let mut rng1 = StdRng::seed_from_u64(seed);
        let mut rng2 = StdRng::seed_from_u64(seed);

        let chances = MutationChances::new(75);

        // Generate actions with same seed
        let actions1 = chances.gen_mutation_actions(&mut rng1);
        let actions2 = chances.gen_mutation_actions(&mut rng2);

        // Should produce identical results
        assert_eq!(actions1.len(), actions2.len());
        for (a1, a2) in actions1.iter().zip(actions2.iter()) {
            assert_eq!(format!("{:?}", a1), format!("{:?}", a2));
        }
    }

    #[test]
    fn test_mutation_action_selection() {
        let mut rng = StdRng::seed_from_u64(7777);

        // Test extreme cases - only split connection
        let chances = MutationChances::new_from_raw(100, 100.0, 0.0, 0.0, 0.0);
        for _ in 0..10 {
            let action = rng.gen_mutation_action(&chances);
            assert!(matches!(action, MutationAction::SplitConnection));
        }

        // Test extreme cases - only mutate exponent
        let chances = MutationChances::new_from_raw(100, 0.0, 0.0, 100.0, 0.0);
        for _ in 0..10 {
            let action = rng.gen_mutation_action(&chances);
            assert!(matches!(action, MutationAction::RemoveNeuron));
        }
    }

    #[test]
    fn test_adjust_mutation_chances_evolution() {
        let mut rng = StdRng::seed_from_u64(3333);
        let mut chances = MutationChances::new(100); // High self-mutation

        let original = chances;
        chances.adjust_mutation_chances(&mut rng);

        // Should have changed something
        let changed = chances.split_connection() != original.split_connection()
            || chances.add_connection() != original.add_connection()
            || chances.remove_connection() != original.remove_connection()
            || chances.mutate_weight() != original.mutate_weight()
            || chances.self_mutation() != original.self_mutation();

        assert!(changed, "High self-mutation should cause changes");

        // Should still sum to 100
        let total = chances.split_connection()
            + chances.add_connection()
            + chances.remove_connection()
            + chances.mutate_weight();
        assert!((total - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_copy_and_equality() {
        let chances1 = MutationChances::new_from_raw(75, 30.0, 25.0, 20.0, 15.0);
        let chances2 = chances1; // Copy

        assert_eq!(chances1, chances2);
        assert_eq!(chances1.self_mutation(), chances2.self_mutation());
        assert_eq!(chances1.split_connection(), chances2.split_connection());
        assert_eq!(chances1.add_connection(), chances2.add_connection());
        assert_eq!(chances1.remove_connection(), chances2.remove_connection());
        assert_eq!(chances1.mutate_weight(), chances2.mutate_weight());
    }
}
*/
