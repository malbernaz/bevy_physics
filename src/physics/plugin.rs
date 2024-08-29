use bevy::{prelude::*, transform::TransformSystem};

use super::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Physics {
    Sync,
    Simulation,
    Debug,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(96.0))
            .register_type::<Collider>()
            .register_type::<Actor>()
            .register_type::<Velocity>()
            .add_event::<CollisionEvent>()
            .configure_sets(
                FixedUpdate,
                (Physics::Sync, Physics::Simulation, Physics::Debug)
                    .chain()
                    .before(TransformSystem::TransformPropagate),
            )
            .add_systems(
                FixedUpdate,
                (
                    update_actor_grounded.in_set(Physics::Sync),
                    move_actor.in_set(Physics::Simulation),
                ),
            );
    }
}

pub struct PhysicsDebugPlugin;

impl Plugin for PhysicsDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, draw_collider_gizmos.in_set(Physics::Debug));
    }
}
