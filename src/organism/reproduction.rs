use rand::{rngs::ThreadRng, Rng as _};

pub enum MutationAction {
    Delete,
    New,
    MutateOrgan,
}

impl MutationAction {
    pub fn rand(rng: &mut ThreadRng) -> Self {
        match rng.gen_range(0..2) {
            0 => MutationAction::Delete,
            1 => MutationAction::New,
            2 => MutationAction::MutateOrgan,
            _ => panic!(),
        }
    }
}
