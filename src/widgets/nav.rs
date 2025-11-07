use bevy::{feathers::theme::ThemedText, prelude::*};

pub fn section_header(text: impl Into<String>) -> impl Bundle {
    (Text::new(text), TextFont::from_font_size(14.), ThemedText)
}
