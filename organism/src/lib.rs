mod cell;
pub use cell::*;

mod genome;
pub use genome::*;

mod organism;
pub use organism::*;

mod network;
pub use network::*;

use bevy::prelude::*;

pub struct OrganismPlugin;

impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((cell::plugin, organism::plugin));
    }
}
