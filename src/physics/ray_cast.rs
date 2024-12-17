use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume, RayCast2d},
    prelude::*,
};

use super::*;

#[derive(Debug, Clone, Copy, Reflect)]
pub struct RayCast {
    pub direction: Cardinal,
    pub length: f32,
}

impl RayCast {
    pub fn new(direction: Cardinal, length: f32) -> Self {
        Self { direction, length }
    }

    pub fn ray_cast(&self, position: Vec2) -> RayCast2d {
        RayCast2d::new(position, self.direction.as_dir().unwrap(), self.length)
    }
}

impl Shape for RayCast {
    fn collides(&self, position: Vec2, aabb: &Aabb2d) -> bool {
        RayCast2d::new(position, self.direction.as_dir().unwrap(), self.length - 1.)
            .intersects(aabb)
    }

    fn get_collision_side(&self, position: Vec2, aabb: &Aabb2d) -> Option<Cardinal> {
        if !self.ray_cast(position).intersects(aabb) {
            return None;
        }

        Some(self.direction)
    }

    fn draw_gizmo(&self, gizmos: &mut Gizmos, position: Vec2, color: Color) {
        let start = position;
        let end = start + self.length * self.direction.as_vec2();

        gizmos.arrow_2d(start, end, color);
    }

    fn as_typed_shape(&self) -> TypedShape {
        TypedShape::Ray(*self)
    }
}
