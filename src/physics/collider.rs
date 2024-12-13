use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
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

        gizmos.primitive_2d(
            &Rectangle::from_size(collider.half_size * 2.0),
            transform.translation.xy(),
            0.,
            color,
        );
    }
}
