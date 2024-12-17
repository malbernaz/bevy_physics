use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct Solid;

#[derive(Bundle)]
pub struct SolidBundle {
    pub solid: Solid,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl SolidBundle {
    pub fn new(pos: Vec2, half_size: Vec2) -> Self {
        Self {
            solid: Solid,
            collider: Collider::aabb(half_size),
            transform: Transform::from_xyz(pos.x, pos.y, 0.),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
