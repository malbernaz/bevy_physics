use bevy::{math::bounding::Aabb2d, prelude::*};
use std::sync::Arc;

use super::*;

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub direction: Cardinal,
    pub solid: Aabb2d,
}

#[derive(Clone)]
pub enum TypedShape {
    Aabb(Aabb),
    Ray(RayCast),
    Custom(CustomCollider),
    None,
}

pub trait Shape: Send + Sync {
    fn collides(&self, position: Vec2, aabb: &Aabb2d) -> bool;

    fn get_collision_side(&self, position: Vec2, aabb: &Aabb2d) -> Option<Cardinal>;

    fn draw_gizmo(&self, gizmos: &mut Gizmos, position: Vec2, color: Color);

    fn as_typed_shape(&self) -> TypedShape {
        TypedShape::None
    }
}

#[derive(Clone, Deref)]
pub struct SharedShape(pub Arc<dyn Shape>);

impl SharedShape {
    pub fn new(shape: impl Shape + 'static) -> Self {
        Self(Arc::new(shape))
    }

    pub fn aabb(half_size: Vec2) -> Self {
        Self::new(Aabb::new(half_size))
    }

    pub fn ray_cast(direction: Cardinal, length: f32) -> Self {
        Self::new(RayCast::new(direction, length))
    }

    pub fn custom(shape: impl Shape + 'static) -> Self {
        Self::new(CustomCollider::new(shape))
    }
}

#[derive(Component, Clone, Deref)]
pub struct Collider(pub SharedShape);

impl Default for Collider {
    fn default() -> Self {
        Self::aabb(Vec2::splat(1.))
    }
}

impl Collider {
    pub fn new(shape: SharedShape) -> Self {
        Self(shape)
    }

    pub fn aabb(half_size: Vec2) -> Self {
        Self::new(SharedShape::aabb(half_size))
    }

    pub fn ray_cast(half_size: Vec2) -> Self {
        Self::new(SharedShape::aabb(half_size))
    }

    pub fn custom(shape: impl Shape + 'static) -> Self {
        Self::new(SharedShape::custom(shape))
    }
}

pub fn draw_collider_gizmos(
    mut gizmos: Gizmos,
    query: Query<(&Collider, Option<&Actor>, &Transform)>,
) {
    for (collider, actor, transform) in &query {
        collider.draw_gizmo(
            &mut gizmos,
            transform.translation.xy(),
            if actor.is_some() {
                Color::srgb(0., 0., 1.)
            } else {
                Color::srgb(0., 1., 0.)
            },
        );
    }
}
