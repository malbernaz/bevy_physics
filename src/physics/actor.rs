use bevy::{math::vec2, prelude::*};

use super::*;

#[derive(Component, Default, Debug)]
pub struct Actor;

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
    pub fn new(pos: Vec2, collider: Collider) -> Self {
        Self {
            collider,
            transform: Transform::from_xyz(pos.x, pos.y, 0.),
            ..default()
        }
    }
}

pub fn simulate_actor_movement(
    time: Res<Time>,
    mut ev_collision: EventWriter<CollisionEvent>,
    mut actor: Query<(Entity, &Collider, &mut Velocity, &mut Transform), With<Actor>>,
    solids: Query<(&Collider, &Transform), (With<Solid>, Without<Actor>)>,
) {
    for (entity, collider, mut velocity, mut transform) in &mut actor {
        let delta = time.delta_seconds();
        let dir = velocity.get_direction();

        let amount_f = velocity.value * delta;
        velocity.remainder += amount_f;
        let mut amount_i = velocity.remainder.as_ivec2();
        velocity.remainder -= amount_i.as_vec2();

        // move x
        'move_x: loop {
            let dir_offset = vec2(dir.x, 0.);

            if dir == Vec2::ZERO {
                break;
            }

            for (solid, solid_transform) in &solids {
                let TypedShape::Aabb(solid) = solid.as_typed_shape() else {
                    break;
                };

                let solid = solid.aabb(solid_transform.translation.xy());

                if collider.collides(transform.translation.xy() + dir_offset, &solid) {
                    ev_collision.send(CollisionEvent {
                        entity,
                        direction: Cardinal::from_vec2(dir_offset).unwrap(),
                        solid,
                    });

                    break 'move_x;
                }
            }

            if amount_i.x == 0 {
                break;
            }

            transform.translation.x += dir.x;
            amount_i.x -= dir.x as i32;
        }

        // move y
        'move_y: loop {
            let dir_offset = vec2(0., dir.y);

            if dir_offset == Vec2::ZERO {
                break;
            }

            for (solid, solid_transform) in &solids {
                let TypedShape::Aabb(solid) = solid.as_typed_shape() else {
                    break;
                };

                let solid = solid.aabb(solid_transform.translation.xy());

                if collider.collides(transform.translation.xy() + dir_offset, &solid) {
                    ev_collision.send(CollisionEvent {
                        entity,
                        direction: Cardinal::from_vec2(dir_offset).unwrap(),
                        solid,
                    });

                    break 'move_y;
                }
            }

            if amount_i.y == 0 {
                break;
            }

            transform.translation.y += dir.y;
            amount_i.y -= dir.y as i32;
        }
    }
}
