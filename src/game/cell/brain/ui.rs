use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::RenderLayers,
    prelude::*,
    render::render_resource::{TextureDimension, TextureFormat, TextureUsages},
};

use crate::{
    camera::RenderLayer,
    game::cell::{ActiveCell, BrainCell, CellVisual},
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_click_brain_cell);
    app.add_observer(spawn_brain_cell_ui);
}

fn on_click_brain_cell(
    ev: On<Pointer<Click>>,
    mut commands: Commands,
    brain_cells: Query<&BrainCell>,
    active_cells: Query<Entity, With<ActiveCell>>,
    visuals: Query<Entity, With<CellVisual>>,
) {
    let entity = ev.entity;
    let Ok(brain_cell) = brain_cells.get(entity) else {
        info!("No brain cell!");
        return;
    };
    for active_cell in active_cells {
        commands.entity(active_cell).remove::<ActiveCell>();
    }
    for visual in visuals {
        commands.entity(visual).despawn();
    }

    commands.entity(entity).insert(ActiveCell);
    commands.trigger(SpawnBrainCellUi);
}
#[derive(Event)]
pub struct SpawnBrainCellUi;

fn spawn_brain_cell_ui(
    _: On<SpawnBrainCellUi>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut image = Image::new_uninit(
        default(),
        TextureDimension::D2,
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::all(),
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);

    let camera = commands
        .spawn((
            CellVisual,
            Camera2d,
            Camera {
                target: image_handle.clone().into(),
                ..default()
            },
            RenderLayers::from(RenderLayer::CELL_VISUAL),
        ))
        .id();

    commands
        .spawn((
            CellVisual,
            Pickable::default(),
            Node {
                position_type: PositionType::Absolute,
                top: px(50),
                left: px(50),
                width: px(400),
                height: px(400),
                border: px(5).all(),
                ..default()
            },
            BorderColor::all(Color::WHITE),
            ViewportNode::new(camera),
        ))
        .observe(on_drag_viewport);
}

fn on_drag_viewport(drag: On<Pointer<Drag>>, mut node_query: Query<&mut Node>) {
    if matches!(drag.button, PointerButton::Secondary) {
        let mut node = node_query.get_mut(drag.entity).unwrap();

        if let (Val::Px(top), Val::Px(left)) = (node.top, node.left) {
            node.left = px(left + drag.delta.x);
            node.top = px(top + drag.delta.y);
        };
    }
}
