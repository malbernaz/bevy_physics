use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use super::*;

#[derive(Debug, Clone, Copy, Reflect)]
pub struct Aabb {
    pub half_size: Vec2,
}

impl Aabb {
    pub fn new(half_size: Vec2) -> Self {
        Self { half_size }
    }

    pub fn aabb(&self, position: Vec2) -> Aabb2d {
        Aabb2d::new(position, self.half_size)
    }
}

impl Shape for Aabb {
    fn collides(&self, position: Vec2, aabb: &Aabb2d) -> bool {
        self.aabb(position)
            .intersects(&aabb.shrink(Vec2::splat(1.)))
    }

    fn get_collision_side(&self, position: Vec2, aabb: &Aabb2d) -> Option<Cardinal> {
        if !self.aabb(position).intersects(aabb) {
            return None;
        }

        let offset = aabb.closest_point(position) - position;

        Some(if offset.y.abs() > offset.x.abs() {
            if offset.y > 0. {
                Cardinal::North
            } else {
                Cardinal::South
            }
        } else if offset.x < 0. {
            Cardinal::West
        } else {
            Cardinal::East
        })
    }

    fn draw_gizmo(&self, gizmos: &mut Gizmos, position: Vec2, color: Color) {
        gizmos.rect_2d(position, 0., self.half_size * 2.0, color);
    }

    fn as_typed_shape(&self) -> TypedShape {
        TypedShape::Aabb(*self)
    }
}
