use nora_neat::{prelude::*, simple_net::network::SimpleNetwork};
use rand::{SeedableRng, rngs::StdRng};

fn main() {
    let mut rng = StdRng::seed_from_u64(38102);
    let mut topology = NetworkTopology::new(2, 2, MutationChances::new(8), &mut rng);

    loop {
        let net = SimpleNetwork::from_topology(&topology);
        let output = net.predict(&[1.0, 1.0]).collect::<Vec<_>>();
        println!("outputs: {output:?}");
        topology = topology.replicate(&mut rng);
    }
}
