use bevy::{
    math::{
        bounding::{Aabb2d, IntersectsVolume},
        vec2,
    },
    prelude::*,
};

use super::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CollisionAxis {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Event)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collision_type: CollisionAxis,
}

#[derive(Component, Reflect, Default, Clone, Copy)]
pub struct Collider {
    pub half_size: Vec2,
}

impl Collider {
    pub fn new(half_size: Vec2) -> Self {
        Self { half_size }
    }

    pub fn aabb(&self, position: Vec2) -> Aabb2d {
        Aabb2d::new(position, self.half_size)
    }

    pub fn collides(&self, position: Vec2, other: &Self, other_position: Vec2) -> bool {
        let aabb = self.aabb(position);
        let other_aabb = other.aabb(other_position);

        aabb.min.x < other_aabb.max.x
            && aabb.max.x > other_aabb.min.x
            && aabb.min.y < other_aabb.max.y
            && aabb.max.y > other_aabb.min.y
    }

    /// gets the minimal distance between the sides of the rect
    pub fn min_diff(&self, position: Vec2, other: &Self, other_position: Vec2) -> Vec2 {
        let aabb = self.aabb(position);
        let other_aabb = other.aabb(other_position);

        let top_diff = (aabb.max.y - other_aabb.min.y).abs();
        let right_diff = (aabb.max.x - other_aabb.min.x).abs();
        let bottom_diff = (aabb.min.y - other_aabb.max.y).abs();
        let left_diff = (aabb.min.x - other_aabb.max.x).abs();

        vec2(right_diff.min(left_diff), top_diff.min(bottom_diff))
    }

    pub fn get_collision_side(
        &self,
        position: Vec2,
        other: &Self,
        other_position: Vec2,
    ) -> Option<CollisionSide> {
        let aabb = self.aabb(position);
        let other_aabb = other.aabb(other_position);

        if !aabb.intersects(&other_aabb) {
            return None;
        }

        let offset = other_aabb.closest_point(position) - position;

        let side = if offset.y.abs() > offset.x.abs() {
            if offset.y > 0. {
                CollisionSide::Top
            } else {
                CollisionSide::Bottom
            }
        } else if offset.x < 0. {
            CollisionSide::Left
        } else {
            CollisionSide::Right
        };

        Some(side)
    }
}

pub fn draw_collider_gizmos(
    mut gizmos: Gizmos,
    query: Query<(&Collider, &Transform, Option<&Actor>)>,
) {
    for (collider, transform, actor) in &query {
        let color = if actor.is_some() {
            Color::srgb(0., 0., 1.)
        } else {
            Color::srgb(0., 1., 0.)
        };

        let h_size = collider.half_size;
        let size = h_size * 2.0;

        let center = transform.translation.xy();

        if actor.is_some() {
            gizmos.line_2d(center, vec2(center.x, center.y + h_size.y), color);
            gizmos.line_2d(center, vec2(center.x + h_size.x, center.y), color);
            gizmos.line_2d(center, vec2(center.x, center.y - h_size.y), color);
            gizmos.line_2d(center, vec2(center.x - h_size.x, center.y), color);
        } else {
            gizmos.primitive_2d(&Rectangle::new(size.x, size.y), center, 0., color);
        }
    }
}
