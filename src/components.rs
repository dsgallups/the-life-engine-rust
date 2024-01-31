use bevy::{ecs::component::Component, math::I64Vec2};

#[derive(Component)]
pub struct MousePosBox;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct Position(pub I64Vec2);

/*
pub struct OrganismBundle {
    position: I64Vec2,

}
*/
