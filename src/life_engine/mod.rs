pub mod cell;
//pub mod messages;
pub mod organism;
pub mod world;

use bevy::render::color::Color;
pub use cell::*;
//pub use messages::*;
pub use organism::*;
pub use world::*;

pub trait Drawable {
    fn color(&self) -> Color;
}
