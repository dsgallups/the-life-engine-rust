mod cell;
pub use cell::*;

//mod old_genome;
//pub use old_genome::*;

pub mod cpu_net;
pub mod genome;

mod organism;
pub use organism::*;

use bevy::prelude::*;

pub struct OrganismPlugin;

impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((cell::plugin, organism::plugin));
    }
}
