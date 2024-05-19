use bevy::prelude::*;

use super::*;

#[derive(Component, Default)]
pub struct Actor;

#[derive(Bundle, Default)]
pub struct ActorBundle {
    pub actor: Actor,
    pub velocity: Velocity,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl ActorBundle {
    pub fn new(pos: Vec2, half_size: Vec2) -> Self {
        Self {
            transform: Transform::from_xyz(pos.x, pos.y, 0.),
            collider: Collider::new(pos, half_size),
            ..default()
        }
    }
}

pub fn calculate_actor_collisions(
    time: Res<Time<Fixed>>,
    mut ev_collision: EventWriter<CollisionEvent>,
    mut actor: Query<(&mut Transform, &mut Velocity, &mut Collider, Entity), With<Actor>>,
    solids: Query<&Collider, (With<Solid>, Without<Actor>)>,
) {
    let Ok((mut transform, mut velocity, mut collider, entity)) = actor.get_single_mut() else {
        return;
    };

    let delta = time.delta_seconds();
    let dir = velocity.get_direction();

    //--------------move x--------------//
    let amount_x = velocity.amount.x * delta;
    velocity.remainder.x += amount_x;
    let mut move_x = velocity.remainder.x as i32;
    velocity.remainder.x -= move_x as f32;

    while move_x != 0 {
        let will_collide = solids
            .iter()
            .find(|&s| collider.collides_at(Vec2::new(move_x as f32, 0.), s));

        if let Some(s_col) = will_collide {
            // correct remaining distance to be traverse
            let diff = collider.min_diff(s_col) * dir;
            collider.update_position_by(Vec2::new(diff.x, 0.));

            ev_collision.send(CollisionEvent {
                entity,
                collision_type: CollisionType::Horizontal,
            });

            break;
        }

        collider.update_position_by(Vec2::new(dir.x, 0.));
        move_x -= dir.x as i32;
    }

    //--------------move y--------------//
    let amount_y = velocity.amount.y * delta;
    velocity.remainder.y += amount_y;
    let mut move_y = velocity.remainder.y as i32;
    velocity.remainder.y -= move_y as f32;

    while move_y != 0 {
        let will_collide = solids
            .iter()
            .find(|&s| collider.collides_at(Vec2::new(0., move_y as f32), s));

        if let Some(s_col) = will_collide {
            // correct remaining distance to be traverse
            let diff = collider.min_diff(s_col) * dir;
            collider.update_position_by(Vec2::new(0., diff.y));

            ev_collision.send(CollisionEvent {
                entity,
                collision_type: CollisionType::Vertical,
            });

            break;
        }

        collider.update_position_by(Vec2::new(0., dir.y));
        move_y -= dir.y as i32;
    }

    let pos = collider.center();
    transform.translation = Vec3::new(pos.x, pos.y, 0.);
}
