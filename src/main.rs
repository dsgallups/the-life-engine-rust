pub mod cell;
//pub mod messages;
pub mod organism;
pub mod world;

mod render;
use render::begin_ticking;

pub use cell::*;
//pub use messages::*;
pub use organism::*;
pub use world::*;

fn main() {
    //println!("ozymandias");
    let mut world = LEWorld::new();
    world.add_simple_organism((0, 0, 0).into());
    loop {
        if let Err(e) = world.tick() {
            println!("error: {}", e);
            break;
        }
    }

    //begin_ticking(world);
}
