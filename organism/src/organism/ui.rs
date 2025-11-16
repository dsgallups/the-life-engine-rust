use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::RenderLayers,
    prelude::*,
    render::render_resource::{TextureDimension, TextureFormat, TextureUsages},
};
use ev_core::{NodeCamera, RenderLayer};

use crate::ActiveOrganism;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_organism_ui);
}

pub(super) fn set_active(
    ev: On<Pointer<Click>>,
    mut commands: Commands,
    actives: Query<Entity, With<ActiveOrganism>>,
) {
    info!("settings active");
    for active in actives {
        commands.entity(active).remove::<ActiveOrganism>();
    }

    commands.entity(ev.entity).insert(ActiveOrganism);
    commands.trigger(SpawnOrganismUi);
}

#[derive(Event)]
pub struct SpawnOrganismUi;

#[derive(Component, Reflect)]
pub struct CellVisual;

fn spawn_organism_ui(
    _: On<SpawnOrganismUi>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
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
            NodeCamera,
            Camera2d,
            Camera {
                target: image_handle.clone().into(),
                ..default()
            },
            Transform::from_scale(Vec3::splat(2.)),
            RenderLayers::from(RenderLayer::NODE_VISUAL),
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
                width: px(800),
                height: px(600),
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
