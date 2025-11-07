use bevy::prelude::*;
use bevy_egui::UiRenderOrder;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::settings::Keybinds;

pub fn gadget(app: &mut App) {
    app.register_type::<AnimationNodeIndex>();

    app.init_resource::<InspectorActive>()
        .add_plugins((
            EguiPlugin {
                ui_render_order: UiRenderOrder::EguiAboveBevyUi,
                ..Default::default()
            },
            WorldInspectorPlugin::new().run_if(is_inspector_active),
        ))
        .add_systems(Update, toggle_inspector);
}

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub(super) struct InspectorActive(bool);

fn is_inspector_active(inspector: Res<InspectorActive>) -> bool {
    inspector.0
}

fn toggle_inspector(
    input: Res<ButtonInput<KeyCode>>,
    keybinds: Res<Keybinds>,
    mut inspector: ResMut<InspectorActive>,
) {
    if input.just_pressed(keybinds.inspector_toggle) {
        info!("Toggled inspector");
        inspector.0 = !inspector.0;
    }
}
