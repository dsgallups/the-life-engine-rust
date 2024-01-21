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
    let mut world = LEWorld::new_walled(100);
    world.add_simple_organism((0, 0, 0).into());

    begin_ticking(world);
}
