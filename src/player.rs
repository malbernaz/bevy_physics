use bevy::{math::bounding::*, prelude::*};

use crate::physics::*;

#[derive(Component)]
pub struct Player {
    speed: Vec2,
    remainder: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: Vec2::ZERO,
            remainder: Vec2::ZERO,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite: SpriteBundle,
    name: Name,
    actor: ActorBundle,
    player: Player,
}

impl PlayerBundle {
    pub fn new(transform: Transform, texture: Handle<Image>) -> Self {
        let center = transform.translation.xy();

        Self {
            sprite: SpriteBundle {
                texture,
                transform,
                ..default()
            },
            name: Name::new("Player"),
            player: Player::default(),
            actor: ActorBundle::new(center, Vec2::new(8. / 2., 16. / 2.)),
        }
    }
}

pub fn get_input_axis(keys: &ButtonInput<KeyCode>, pos: KeyCode, neg: KeyCode) -> f32 {
    let pos = if keys.pressed(pos) { 1. } else { 0. };
    let neg = if keys.pressed(neg) { 1. } else { 0. };
    pos - neg
}

pub fn approach(value: f32, target: f32, delta: f32) -> f32 {
    if value > target {
        return target.max(value - delta);
    }
    target.min(value + delta)
}

const SPEED: f32 = 150.;
const ACC: f32 = 10.;
const GRAVITY: f32 = 10.;
const FALL_SPEED: f32 = -400.;
const JUMP_SPEED: f32 = 300.;

pub fn movement(keys: Res<ButtonInput<KeyCode>>, mut player: Query<&mut Player>) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let x_axis = get_input_axis(&keys, KeyCode::ArrowRight, KeyCode::ArrowLeft);
    let _y_axis = get_input_axis(&keys, KeyCode::ArrowUp, KeyCode::ArrowDown);

    player.speed.x = approach(player.speed.x, SPEED * x_axis, ACC);
    player.speed.y = approach(player.speed.y, FALL_SPEED, GRAVITY);

    if keys.just_pressed(KeyCode::KeyC) {
        player.speed.y = JUMP_SPEED;
    }

    player.speed;
}

pub fn aabb_from_movement(rect: Aabb2d, movement: Vec2) -> Aabb2d {
    let center = rect.center();
    let half_size = rect.half_size();
    Aabb2d::new(center + movement, half_size)
}

pub fn collision_system(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut Player, &Collider), With<Player>>,
    solids: Query<&Collider, (With<Solid>, Without<Player>)>,
) {
    let Ok((mut p_trans, mut player, p_col)) = player.get_single_mut() else {
        return;
    };

    let delta = time.delta_seconds();

    'x_move: {
        let amount_x = player.speed.x * delta;
        player.remainder.x += amount_x;
        let mut move_x = player.remainder.x as i32;

        if move_x != 0 {
            player.remainder.x -= move_x as f32;
            let sign = move_x.signum();

            while move_x != 0 {
                let next_rect = aabb_from_movement(p_col.rect, Vec2::new(move_x as f32, 0.))
                    // don't bother for collisions on y
                    .shrink(Vec2::new(0., 1.));
                let will_collide = solids
                    .iter()
                    .map(|s| s.rect)
                    .find(|&s_rect| next_rect.intersects(&s_rect));

                if let Some(s_rect) = will_collide {
                    let u_diff = (p_col.rect.max.x - s_rect.min.x).abs();
                    let d_diff = (p_col.rect.min.x - s_rect.max.x).abs();
                    let diff = u_diff.min(d_diff) * sign as f32;

                    if diff != 0. {
                        p_trans.translation.x += diff;
                    }

                    player.speed.x = 0.;
                    player.remainder.x = 0.;

                    break;
                }

                p_trans.translation.x += sign as f32;
                move_x -= sign;
            }
        }
    }

    'moveY: {
        let amount_y = player.speed.y * delta;
        player.remainder.y += amount_y;
        let mut move_y = player.remainder.y as i32;

        if move_y != 0 {
            player.remainder.y -= move_y as f32;
            let sign = move_y.signum();

            while move_y != 0 {
                let next_rect = aabb_from_movement(p_col.rect, Vec2::new(0., move_y as f32))
                    // don't bother for collisions on x
                    .shrink(Vec2::new(1., 0.));
                let will_collide = solids
                    .iter()
                    .map(|s| s.rect)
                    .find(|&s_rect| next_rect.intersects(&s_rect));

                if let Some(s_rect) = will_collide {
                    let u_diff = (p_col.rect.max.y - s_rect.min.y).abs();
                    let d_diff = (p_col.rect.min.y - s_rect.max.y).abs();
                    let diff = u_diff.min(d_diff) * sign as f32;

                    if diff != 0. {
                        p_trans.translation.y += diff;
                    }

                    player.speed.y = 0.;
                    player.remainder.y = 0.;

                    break;
                }

                p_trans.translation.y += sign as f32;
                move_y -= sign;
            }
        }
    }
}
