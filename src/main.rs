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
    begin_ticking();
}
