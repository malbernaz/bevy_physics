use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct Solid;

#[derive(Bundle)]
pub struct SolidBundle {
    solid: Solid,
    collider: Collider,
}

impl SolidBundle {
    pub fn new(center: Vec2, half_size: Vec2) -> Self {
        let collider = Collider::new(center, half_size);

        Self {
            solid: Solid,
            collider,
        }
    }
}
