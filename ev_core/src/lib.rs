mod node;
pub use node::*;

mod render_layers;
pub use render_layers::*;

use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(node::plugin);
    }
}
