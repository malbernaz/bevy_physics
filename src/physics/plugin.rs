use bevy::prelude::*;

use super::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Collider>()
            .register_type::<Velocity>()
            .add_event::<CollisionEvent>()
            .add_systems(FixedUpdate, calculate_actor_collisions);
    }
}

pub struct PhysicsDebugPlugin;

impl Plugin for PhysicsDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_collider_gizmos);
    }
}
