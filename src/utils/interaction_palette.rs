use bevy::prelude::*;

use crate::utils::{BUTTON_BACKGROUND, BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();

    app.add_observer(add_interaction_observer)
        .add_systems(Update, apply_interaction_palette);
}

#[derive(Component, Debug, Reflect)]
pub struct InteractionPalette {
    none: Color,
    hovered: Color,
    pressed: Color,
}

impl InteractionPalette {
    pub fn new(base: impl Into<Color>) -> Self {
        let base_color = base.into();

        // Convert to HSL to adjust lightness
        let Hsla {
            hue,
            saturation,
            lightness,
            alpha,
        } = base_color.into();

        // Determine hover and pressed colors based on the base lightness
        let (hovered_color, pressed_color) = if lightness > 0.85 {
            // Very light colors: darken for both states
            let hovered = Color::hsla(hue, saturation, lightness - 0.10, alpha);
            let pressed = Color::hsla(hue, saturation, lightness - 0.25, alpha);
            (hovered, pressed)
        } else if lightness < 0.15 {
            // Very dark colors: lighten for both states
            let hovered = Color::hsla(hue, saturation, lightness + 0.25, alpha);
            let pressed = Color::hsla(hue, saturation, lightness + 0.10, alpha);
            (hovered, pressed)
        } else {
            // Normal colors: lighten for hover, darken for pressed
            let hovered = Color::hsla(hue, saturation, (lightness + 0.15).min(1.0), alpha);
            let pressed = Color::hsla(hue, saturation, (lightness - 0.20).max(0.0), alpha);
            (hovered, pressed)
        };

        Self {
            none: base_color,
            hovered: hovered_color,
            pressed: pressed_color,
        }
    }
}

impl Default for InteractionPalette {
    fn default() -> Self {
        Self {
            none: BUTTON_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        }
    }
}

fn add_interaction_observer(
    trigger: On<Add, InteractionPalette>,
    mut commands: Commands,
    palettes: Query<&InteractionPalette>,
) {
    let palette = palettes.get(trigger.event_target()).unwrap();
    commands
        .entity(trigger.event_target())
        .insert_if_new(BackgroundColor(palette.none));
}

fn apply_interaction_palette(
    palettes: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in palettes {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}
