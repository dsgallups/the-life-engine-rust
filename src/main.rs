#![allow(dead_code)]

use std::{thread, time};

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

fn main() {
    //println!("ozymandias");
    let mut world = LEWorld::new();
    world.add_simple_organism((0, 0, 0).into());

    let mut count = 0;
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        let _ = world.tick();
        println!("TICK {}", count);
        count += 1;
        if count == 100 {
            break;
        }
    }
}
