use std::sync::Arc;

use bevy::{math::bounding::Aabb2d, prelude::*};

use super::*;

#[derive(Clone, Deref)]
pub struct CustomCollider(pub Arc<dyn Shape>);

impl CustomCollider {
    pub fn new(shape: impl Shape + 'static) -> Self {
        Self(Arc::new(shape))
    }
}

impl Shape for CustomCollider {
    fn collides(&self, position: Vec2, aabb: &Aabb2d) -> bool {
        self.0.collides(position, aabb)
    }

    fn get_collision_side(&self, position: Vec2, aabb: &Aabb2d) -> Option<Cardinal> {
        self.0.get_collision_side(position, aabb)
    }

    fn draw_gizmo(&self, gizmos: &mut Gizmos, position: Vec2, color: Color) {
        self.0.draw_gizmo(gizmos, position, color);
    }

    fn as_typed_shape(&self) -> TypedShape {
        TypedShape::Custom(self.clone())
    }
}
