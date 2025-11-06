#[cfg(feature = "dev")]
mod dev_tools;

mod camera;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugins((camera::plugin));

    #[cfg(feature = "dev")]
    app.add_plugins(dev_tools::plugin);

    app.run();
}
