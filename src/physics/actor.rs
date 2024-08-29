use bevy::{math::vec2, prelude::*};

use super::*;

#[derive(Component, Reflect, Default, Debug)]
pub struct Actor {
    pub grounded: bool,
}

#[derive(Bundle, Default)]
pub struct ActorBundle {
    pub actor: Actor,
    pub collider: Collider,
    pub velocity: Velocity,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl ActorBundle {
    pub fn new(pos: Vec2, half_size: Vec2) -> Self {
        Self {
            collider: Collider::new(half_size),
            transform: Transform::from_xyz(pos.x, pos.y, 0.),
            ..default()
        }
    }
}

pub fn move_actor(
    time: Res<Time>,
    mut ev_collision: EventWriter<CollisionEvent>,
    mut actor: Query<(Entity, &Collider, &mut Velocity, &mut Transform), With<Actor>>,
    solids: Query<(&Collider, &Transform), (With<Solid>, Without<Actor>)>,
) {
    for (entity, collider, mut velocity, mut transform) in &mut actor {
        let delta = time.delta_seconds();
        let dir = velocity.get_direction();

        //--------------move x--------------//
        let amount_x = velocity.value.x * delta;

        velocity.remainder.x += amount_x;
        let mut move_x = velocity.remainder.x as i32;
        velocity.remainder.x -= move_x as f32;

        while move_x != 0 {
            let will_collide = solids.iter().find(|(solid_collider, solid_translation)| {
                collider.collides(
                    transform.translation.xy() + vec2(move_x as f32, 0.),
                    solid_collider,
                    solid_translation.translation.xy(),
                )
            });

            if let Some((solid_collider, solid_translation)) = will_collide {
                // correct remaining distance to be traversed
                let diff = collider.min_diff(
                    transform.translation.xy(),
                    solid_collider,
                    solid_translation.translation.xy(),
                ) * dir;

                transform.translation.x += diff.x;

                ev_collision.send(CollisionEvent {
                    entity,
                    collision_type: CollisionAxis::Horizontal,
                });

                break;
            }

            transform.translation.x += dir.x;

            move_x -= dir.x as i32;
        }

        //--------------move y--------------//
        let amount_y = velocity.value.y * delta;
        velocity.remainder.y += amount_y;
        let mut move_y = velocity.remainder.y as i32;
        velocity.remainder.y -= move_y as f32;

        while move_y != 0 {
            let will_collide = solids.iter().find(|(solid_collider, solid_transform)| {
                collider.collides(
                    transform.translation.xy() + vec2(0., move_y as f32),
                    solid_collider,
                    solid_transform.translation.xy(),
                )
            });

            if let Some((solid_collider, solid_transform)) = will_collide {
                // correct remaining distance to be traversed
                let diff = collider.min_diff(
                    transform.translation.xy(),
                    solid_collider,
                    solid_transform.translation.xy(),
                ) * dir;

                transform.translation.y += diff.y;

                ev_collision.send(CollisionEvent {
                    entity,
                    collision_type: CollisionAxis::Vertical,
                });

                break;
            }

            transform.translation.y += dir.y;

            move_y -= dir.y as i32;
        }
    }
}

pub fn update_actor_grounded(
    mut actor: Query<(&mut Actor, &Collider, &Transform)>,
    solids: Query<(&Collider, &Transform), (With<Solid>, Without<Actor>)>,
) {
    for (mut actor, collider, transform) in &mut actor {
        actor.grounded = solids.iter().any(|(solid_collider, solid_transform)| {
            if let Some(side) = collider.get_collision_side(
                transform.translation.xy(),
                solid_collider,
                solid_transform.translation.xy(),
            ) {
                if side != CollisionSide::Bottom {
                    return false;
                }

                let (aabb, other_aabb) = (
                    collider.aabb(transform.translation.xy()),
                    solid_collider.aabb(solid_transform.translation.xy()),
                );

                return aabb.max.x != other_aabb.min.x && aabb.min.x != other_aabb.max.x;
            }

            false
        });
    }
}
