pub mod cell;
//pub mod messages;
pub mod organism;
pub mod world;

mod event;

mod render;

use render::begin_ticking;

pub use cell::*;
//pub use messages::*;
pub use event::*;
pub use organism::*;
pub use world::*;

fn main() {
    let mut world = LEWorld::new();
    world.add_simple_producer((0, 0).into());

    //world.add_simple_mover((5, 0, 0).into());

    begin_ticking(world);
}
