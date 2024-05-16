use bevy::{
    math::bounding::{Aabb2d, BoundingVolume},
    prelude::*,
};

use super::*;

#[derive(Component)]
pub struct Collider {
    pub rect: Aabb2d,
}

impl Collider {
    pub fn new(center: Vec2, half_size: Vec2) -> Self {
        Self {
            rect: Aabb2d::new(center, half_size),
        }
    }

    pub fn update_rect(&mut self, center: Vec2) {
        self.rect = Aabb2d::new(center, self.rect.half_size());
    }
}

pub fn update_rect(
    mut gizmos: Gizmos,
    mut query: Query<(&mut Collider, &Transform, Option<&Actor>)>,
) {
    for (mut collider, transform, actor) in &mut query {
        if actor.is_some() {
            collider.update_rect(transform.translation.xy());
        }

        gizmos.primitive_2d(
            Rectangle::from_corners(collider.rect.min, collider.rect.max),
            collider.rect.center(),
            0.,
            Color::rgb(0., 1., 0.),
        );
    }
}
