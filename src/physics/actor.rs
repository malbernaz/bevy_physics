use bevy::prelude::*;

use super::collider::Collider;

#[derive(Component)]
pub struct Actor;

#[derive(Bundle)]
pub struct ActorBundle {
    collider: Collider,
    actor: Actor,
}

impl ActorBundle {
    pub fn new(collider: Collider) -> Self {
        Self {
            collider,
            actor: Actor,
        }
    }
}
