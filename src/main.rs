use bevy::prelude::*;

fn main() -> AppExit {
    App::new().add_plugins(life_engine_rs::plugin).run()
}
