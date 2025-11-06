//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::prelude::*;

mod debug;
mod inspector;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((inspector::gadget, debug::plugin));
    // Log `Screen` state transitions.
    // app.add_systems(
    //     Update,
    //     (
    //         log_transitions::<Screen>,
    //         log_transitions::<LethalState>,
    //         log_transitions::<Menu>,
    //         log_transitions::<GameMode>,
    //     ),
    // );
    app.add_observer(pick_dbg);
    app.add_observer(debug);
    //.add_systems(Update, set_cam);
}

fn pick_dbg(ev: On<Pointer<Click>>, names: Query<&Name>) {
    let name = names
        .get(ev.event_target())
        .map(|n| n.to_string())
        .unwrap_or("Unknown".to_string());

    info!("Picked {name}({:?})", ev.event_target());
}
#[derive(EntityEvent)]
pub struct DebugEntity {
    entity: Entity,
}
#[allow(dead_code)]
impl DebugEntity {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

fn debug(ev: On<DebugEntity>, names: Query<&Name>) {
    let name = names
        .get(ev.event_target())
        .map(|n| n.to_string())
        .unwrap_or("Unknown".to_string());

    info!("DEBUG: {}: {name}", ev.event_target());
}

// fn set_cam(
//     mut commands: Commands,
//     input: Res<ButtonInput<KeyCode>>,
//     keybinds: Res<Keybinds>,
//     mut toggle: Local<usize>,
//     cam: Query<Entity, With<WorldCamera>>,
// ) {
//     if !input.just_pressed(keybinds.camera_switch) {
//         return;
//     }

//     *toggle += 1;
//     *toggle = (*toggle + 1) % 3;

//     let to = match *toggle {
//         0 => DEFAULT_LOCATIONS.piano(),
//         1 => Transform::from_translation(Vec3::new(0., 1., -3.)).looking_at(Vec3::ZERO, Vec3::Y),
//         _ => DEFAULT_LOCATIONS.overview(),
//     };

//     commands.trigger(MoveCamera::new(
//         cam.single().unwrap(),
//         to,
//         Duration::from_secs(3),
//     ));
// }
