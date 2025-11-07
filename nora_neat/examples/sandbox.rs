use nora_neat::prelude::*;
use rand::{SeedableRng, rngs::StdRng};

fn main() {
    let mut rng = StdRng::seed_from_u64(38102);
    let topology = NetworkTopology::new(2, 2, MutationChances::new(4), &mut rng);
}
