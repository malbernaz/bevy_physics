use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct Solid;

#[derive(Bundle)]
pub struct SolidBundle {
    solid: Solid,
    collider: Collider,
    transform: TransformBundle,
}

impl SolidBundle {
    pub fn new(pos: Vec2, half_size: Vec2) -> Self {
        Self {
            solid: Solid,
            transform: TransformBundle::from_transform(Transform::from_xyz(pos.x, pos.y, 0.)),
            collider: Collider::new(pos, half_size),
        }
    }
}
