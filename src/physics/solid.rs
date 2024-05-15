use bevy::prelude::*;

use super::collider::Collider;

#[derive(Component)]
pub struct Solid;

#[derive(Bundle)]
pub struct SolidBundle {
    solid: Solid,
    collider: Collider,
}

impl SolidBundle {
    pub fn new(collider: Collider) -> Self {
        Self {
            solid: Solid,
            collider,
        }
    }
}
