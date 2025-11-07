use bevy::prelude::*;
use std::hash::Hash;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Keybinds::dan());
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct Keybinds {
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub button_rotate: MouseButton,
    pub key_rotate_left: KeyCode,
    pub key_rotate_right: KeyCode,

    #[cfg(feature = "dev")]
    pub debug_toggle: KeyCode,
    #[cfg(feature = "dev")]
    pub inspector_toggle: KeyCode,

    #[cfg(feature = "dev")]
    pub fps_toggle: KeyCode,

    pub demo_mode: KeyCode,
}

impl Keybinds {
    fn dan() -> Self {
        Self {
            key_up: KeyCode::KeyI,
            key_down: KeyCode::KeyK,
            key_left: KeyCode::KeyJ,
            key_right: KeyCode::KeyL,
            button_rotate: MouseButton::Middle,
            key_rotate_left: KeyCode::KeyU,
            key_rotate_right: KeyCode::KeyO,
            #[cfg(feature = "dev")]
            debug_toggle: KeyCode::KeyY,
            #[cfg(feature = "dev")]
            inspector_toggle: KeyCode::KeyH,
            #[cfg(feature = "dev")]
            fps_toggle: KeyCode::KeyN,
            demo_mode: KeyCode::KeyM,
        }
    }
}
#[expect(dead_code)]
pub fn keyb_just_pressed<Key>(
    keycode: impl Fn(&Keybinds) -> Key,
) -> impl Fn(Res<ButtonInput<Key>>, Res<Keybinds>) -> bool
where
    Key: Send + Sync + Eq + Hash + Copy + 'static,
{
    move |input: Res<ButtonInput<Key>>, keybinds: Res<Keybinds>| {
        input.just_pressed(keycode(&keybinds))
    }
}
