//! Toggles for the different debug UIs that our plugins provide.

use crate::camera::RenderLayer;
use crate::settings::Keybinds;
use crate::widgets;
use avian3d::prelude::{
    PhysicsDebugPlugin, PhysicsDiagnosticsPlugin, PhysicsDiagnosticsUiPlugin,
    PhysicsDiagnosticsUiSettings, PhysicsGizmos,
};
use bevy::camera::visibility::RenderLayers;
use bevy::dev_tools::fps_overlay::FrameTimeGraphConfig;
use bevy::ui::Val::*;
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};
use bevy_reagent::gizmo::AgentGizmoConfig;
//use bevy_landmass::debug::{EnableLandmassDebug, Landmass3dDebugPlugin, LandmassGizmos};
use bevy_rerecast::debug::NavmeshGizmoConfig;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DebugSet {
    RecordInput,
    Update,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DebugState>();

    app.add_plugins((
        PhysicsDebugPlugin,
        PhysicsDiagnosticsPlugin,
        PhysicsDiagnosticsUiPlugin,
    ));

    app.insert_resource(PhysicsDiagnosticsUiSettings {
        enabled: false,
        ..default()
    });

    app.insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            render_layers: RenderLayers::from(RenderLayer::GIZMO3),
            ..default()
        },
    );

    app.insert_gizmo_config(
        LightGizmoConfigGroup {
            draw_all: true,
            ..default()
        },
        GizmoConfig {
            enabled: false,
            render_layers: RenderLayers::from(RenderLayer::GIZMO3),
            ..default()
        },
    );

    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            enabled: false,
            frame_time_graph_config: FrameTimeGraphConfig {
                enabled: false,
                ..default()
            },
            ..default()
        },
    });
    app.insert_resource(NavmeshGizmoConfig {
        polygon_navmesh: GizmoConfig {
            enabled: false,
            ..default()
        },
        detail_navmesh: GizmoConfig {
            enabled: false,
            ..default()
        },
    });
    app.insert_resource(AgentGizmoConfig {
        path: GizmoConfig {
            enabled: false,
            ..default()
        },
    });

    // app.add_plugins(Landmass3dDebugPlugin {
    //     draw_on_start: false,
    //     ..default()
    // });

    // app.insert_gizmo_config(
    //     LandmassGizmos {
    //         boundary_edge: None,
    //         connectivity_edge: None,
    //         ..default()
    //     },
    //     GizmoConfig {
    //         enabled: true,
    //         depth_bias: -0.03,
    //         render_layers: RenderLayers::from(RenderLayer::GIZMO3),
    //         ..default()
    //     },
    // );

    app.configure_sets(Update, (DebugSet::RecordInput, DebugSet::Update).chain());

    app.add_systems(Startup, setup_debug_ui_text);
    app.add_systems(
        Update,
        update_debug_ui_text.run_if(resource_exists_and_changed::<DebugState>),
    )
    .add_systems(Update, advance_debug_state.in_set(DebugSet::RecordInput))
    .add_systems(
        Update,
        (
            toggle_fps_overlay,
            toggle_debug_ui.run_if(toggled_state(DebugState::Ui)),
            toggle_physics_debug_ui.run_if(toggled_state(DebugState::Physics)),
            toggle_agent_debug_ui.run_if(toggled_state(DebugState::Agent)),
            toggle_lighting_debug_ui.run_if(toggled_state(DebugState::Lighting)),
        )
            .in_set(DebugSet::Update)
            .chain(),
    );
}

fn setup_debug_ui_text(mut commands: Commands) {
    commands.spawn((
        Name::new("Debug UI"),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            justify_content: JustifyContent::End,
            ..default()
        },
        children![(widgets::label("Debug UI"), DebugUiText)],
    ));
}

#[derive(Component)]
struct DebugUiText;

fn advance_debug_state(
    button_inputs: Res<ButtonInput<KeyCode>>,
    keybinds: Res<Keybinds>,
    mut debug_state: ResMut<DebugState>,
) {
    if button_inputs.just_pressed(keybinds.debug_toggle) {
        *debug_state = debug_state.next();
    }
}

fn update_debug_ui_text(
    debug_state: Res<DebugState>,
    mut text: Single<&mut Text, With<DebugUiText>>,
) {
    text.0 = match *debug_state {
        DebugState::None => "",
        DebugState::Ui => "Ui",
        DebugState::Physics => "Physics",
        DebugState::Lighting => "Lighting",
        DebugState::Agent => "Nav + Agent",
    }
    .to_string();
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_fps_overlay(
    mut config: ResMut<FpsOverlayConfig>,
    button_inputs: Res<ButtonInput<KeyCode>>,
    keybinds: Res<Keybinds>,
) {
    if button_inputs.just_pressed(keybinds.fps_toggle) {
        config.enabled = !config.enabled;
    }
}

#[derive(Resource, Debug, Default, Eq, PartialEq, Clone, Copy)]
pub(super) enum DebugState {
    #[default]
    None,
    Ui,
    Physics,
    Agent,
    Lighting,
}

impl DebugState {
    fn next(&self) -> Self {
        match self {
            Self::None => Self::Ui,
            Self::Ui => Self::Physics,
            Self::Physics => Self::Agent,
            Self::Agent => Self::Lighting,
            Self::Lighting => Self::None,
        }
    }
}

fn toggle_physics_debug_ui(
    mut config_store: ResMut<GizmoConfigStore>,
    mut physics_diagnostics: ResMut<PhysicsDiagnosticsUiSettings>,
) {
    let config = config_store.config_mut::<PhysicsGizmos>().0;
    config.enabled = !config.enabled;
    physics_diagnostics.enabled = !physics_diagnostics.enabled;
}

fn toggle_agent_debug_ui(
    mut agent: ResMut<AgentGizmoConfig>,
    mut navmesh: ResMut<NavmeshGizmoConfig>,
) {
    // **debug = !**debug;
    agent.path.enabled = !agent.path.enabled;
    navmesh.detail_navmesh.enabled = !navmesh.detail_navmesh.enabled;
}

fn toggle_lighting_debug_ui(mut config_store: ResMut<GizmoConfigStore>) {
    let config = config_store.config_mut::<LightGizmoConfigGroup>().0;
    config.enabled = !config.enabled;
}

pub(super) fn toggled_state(state: DebugState) -> impl SystemCondition<()> {
    IntoSystem::into_system(move |current_state: Res<DebugState>| {
        let was_just_changed = current_state.is_changed() && !current_state.is_added();
        let entered_state = *current_state == state;
        let exited_state = *current_state == state.next();
        was_just_changed && (entered_state || exited_state)
    })
}
