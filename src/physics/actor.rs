use bevy::prelude::*;

use super::Collider;

#[derive(Component)]
pub struct Actor;

#[derive(Bundle)]
pub struct ActorBundle {
    collider: Collider,
    actor: Actor,
}

impl ActorBundle {
    pub fn new(center: Vec2, half_size: Vec2) -> Self {
        Self {
            collider: Collider::new(center, half_size),
            actor: Actor,
        }
    }
}
