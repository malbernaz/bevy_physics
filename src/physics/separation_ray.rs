use bevy::{math::bounding::RayCast2d, prelude::*};

use super::*;

#[derive(Component)]
pub struct SeparationRay {
    pub direction: Dir2,
    pub length: f32,
}

impl SeparationRay {
    pub fn new(direction: Dir2, length: f32) -> Self {
        Self { direction, length }
    }

    pub fn collides(&self, position: Vec2, collider: &Collider, collider_position: Vec2) -> bool {
        let ray_cast = RayCast2d::new(position, self.direction, self.length);
        let target = ray_cast.ray.get_point(self.length);
        let aabb = collider.aabb(collider_position);

        target.x > aabb.min.x
            && target.x < aabb.max.x
            && target.y > aabb.min.y
            && target.y < aabb.max.y
    }
}

pub fn draw_separation_ray_gizmos(
    mut gizmos: Gizmos,
    query: Query<(&SeparationRay, &Transform), With<Actor>>,
) {
    for (ray, transform) in &query {
        let start = transform.translation.xy();
        let end = start + ray.length * ray.direction;
        gizmos.arrow_2d(start, end, Color::srgb(0., 0., 1.));
    }
}
