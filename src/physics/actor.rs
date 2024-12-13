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

pub fn calculate_actor_movement(
    time: Res<Time>,
    mut ev_collision: EventWriter<CollisionEvent>,
    mut actor: Query<
        (
            Entity,
            &Collider,
            &SeparationRay,
            &mut Velocity,
            &mut Transform,
        ),
        With<Actor>,
    >,
    solids: Query<(&Collider, &Transform), (With<Solid>, Without<Actor>)>,
) {
    for (entity, collider, ray, mut velocity, mut transform) in &mut actor {
        let delta = time.delta_seconds();
        let dir = velocity.get_direction();

        let amount_f = velocity.value * delta;
        velocity.remainder += amount_f;
        let mut amount_i = velocity.remainder.as_ivec2();
        velocity.remainder -= amount_i.as_vec2();

        // move x
        loop {
            if ray.direction.x != 0. {
                let ray_collision = solids.iter().find(|(solid_collider, solid_translation)| {
                    ray.collides(
                        transform.translation.xy() + vec2(amount_i.x as f32, 0.),
                        solid_collider,
                        solid_translation.translation.xy(),
                    )
                });

                if let Some((solid_collider, solid_transform)) = ray_collision {
                    let collider_aabb = solid_collider.aabb(solid_transform.translation.xy());

                    transform.translation.x = if ray.direction.x.is_sign_positive() {
                        collider_aabb.min.x - ray.length
                    } else {
                        collider_aabb.max.x + ray.length
                    };

                    ev_collision.send(CollisionEvent {
                        entity,
                        collision_type: CollisionAxis::Horizontal,
                    });

                    break;
                }
            }

            let aabb_collision = solids.iter().find(|(solid_collider, solid_transform)| {
                collider.collides(
                    transform.translation.xy() + vec2(amount_i.x as f32, 0.),
                    solid_collider,
                    solid_transform.translation.xy(),
                )
            });

            if let Some((solid_collider, solid_transform)) = aabb_collision {
                let collider_aabb = solid_collider.aabb(solid_transform.translation.xy());

                transform.translation.x = if dir.x.is_sign_positive() {
                    collider_aabb.min.x - collider.half_size.x
                } else {
                    collider_aabb.max.x + collider.half_size.x
                };

                ev_collision.send(CollisionEvent {
                    entity,
                    collision_type: CollisionAxis::Horizontal,
                });

                break;
            }

            if amount_i.x == 0 {
                break;
            }

            transform.translation.x += dir.x;
            amount_i.x -= dir.x as i32;
        }

        // move y
        loop {
            if ray.direction.y != 0. {
                let ray_collision = solids.iter().find(|(solid_collider, solid_transform)| {
                    ray.collides(
                        transform.translation.xy() + vec2(0., amount_i.y as f32),
                        solid_collider,
                        solid_transform.translation.xy(),
                    )
                });

                if let Some((solid_collider, solid_transform)) = ray_collision {
                    let collider_aabb = solid_collider.aabb(solid_transform.translation.xy());

                    transform.translation.y = if ray.direction.y.is_sign_positive() {
                        collider_aabb.min.y - ray.length
                    } else {
                        collider_aabb.max.y + ray.length
                    };

                    ev_collision.send(CollisionEvent {
                        entity,
                        collision_type: CollisionAxis::Vertical,
                    });

                    break;
                }
            }

            let aabb_collision = solids.iter().find(|(solid_collider, solid_transform)| {
                collider.collides(
                    transform.translation.xy() + vec2(0., amount_i.y as f32),
                    solid_collider,
                    solid_transform.translation.xy(),
                )
            });

            if let Some((solid_collider, solid_transform)) = aabb_collision {
                let collider_aabb = solid_collider.aabb(solid_transform.translation.xy());

                transform.translation.y = if dir.y.is_sign_positive() {
                    collider_aabb.min.y - collider.half_size.y
                } else {
                    collider_aabb.max.y + collider.half_size.y
                };

                ev_collision.send(CollisionEvent {
                    entity,
                    collision_type: CollisionAxis::Vertical,
                });

                break;
            }

            if amount_i.y == 0 {
                break;
            }

            transform.translation.y += dir.y;
            amount_i.y -= dir.y as i32;
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
