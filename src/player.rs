use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::physics::*;

#[derive(Component)]
pub struct Player {
    speed: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: Vec2::ZERO }
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

const SPEED: f32 = 200.;
const ACC: f32 = 8.;
const GRAVITY: f32 = 10.;
const FALL_SPEED: f32 = -1000.;
const JUMP_SPEED: f32 = 200.;

pub fn movement(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut player: Query<&mut Player>) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let x_axis = get_input_axis(&keys, KeyCode::ArrowRight, KeyCode::ArrowLeft);
    let y_axis = get_input_axis(&keys, KeyCode::ArrowUp, KeyCode::ArrowDown);

    let delta = time.delta_seconds();

    player.speed.x = 75. * x_axis * delta;
    player.speed.y = 75. * y_axis * delta;

    // player.speed.x = approach(player.speed.x, SPEED * x_axis * delta, ACC * delta);
    // player.speed.y = approach(player.speed.y, FALL_SPEED * delta, GRAVITY * delta);

    // if keys.pressed(KeyCode::KeyC) {
    //     player.speed.y = JUMP_SPEED * delta;
    // }
}

pub fn collision_system(
    mut player: Query<(&mut Transform, &mut Player, &Collider), With<Player>>,
    solids: Query<&Collider, (With<Solid>, Without<Player>)>,
) {
    let Ok((mut p_trans, mut player, p_col)) = player.get_single_mut() else {
        return;
    };

    let speed = player.speed.round();

    for s_col in &solids {
        let Some((e1, e2)) = get_toc_entries(&p_col.rect, &s_col.rect, speed) else {
            p_trans.translation.x += speed.x;
            p_trans.translation.y += speed.y;
            break;
        };

        let toc = e1.max(e2);
        let collision_axis = collision_direction(e1, e2, speed);

        // println!("{:?}, {:?}", (e1, e2), toc);

        if collision_axis.y != 0. {
            p_trans.translation.x += speed.x;
            p_trans.translation.y += speed.y * toc;
            player.speed.y = 0.;
            break;
        }

        if collision_axis.x != 0. {
            p_trans.translation.y += speed.y;
            p_trans.translation.x += speed.x * toc;
            player.speed.x = 0.;
            break;
        }
    }
}

/// get time of collision entries
fn get_toc_entries(a: &Aabb2d, s: &Aabb2d, speed: Vec2) -> Option<(f32, f32)> {
    let (x_entry, x_exit) = if speed.x > 0.0 {
        ((s.min.x - a.max.x) / speed.x, (s.max.x - a.min.x) / speed.x)
    } else if speed.x < 0.0 {
        ((s.max.x - a.min.x) / speed.x, (s.min.x - a.max.x) / speed.x)
    } else {
        (f32::NEG_INFINITY, f32::INFINITY)
    };

    let (y_entry, y_exit) = if speed.y > 0.0 {
        ((s.min.y - a.max.y) / speed.y, (s.max.y - a.min.y) / speed.y)
    } else if speed.y < 0.0 {
        ((s.max.y - a.min.y) / speed.y, (s.min.y - a.max.y) / speed.y)
    } else {
        (f32::NEG_INFINITY, f32::INFINITY)
    };

    let entry_time = x_entry.max(y_entry);
    let exit_time = x_exit.min(y_exit);

    if entry_time > exit_time || entry_time < 0.0 || entry_time > 1.0 {
        None
    } else {
        Some((x_entry, y_entry))
    }
}

fn collision_direction(x_entry: f32, y_entry: f32, speed: Vec2) -> Vec2 {
    if x_entry > y_entry {
        if speed.x > 0.0 {
            Vec2 { x: 1.0, y: 0.0 } // Collision on the positive X axis
        } else {
            Vec2 { x: -1.0, y: 0.0 } // Collision on the negative X axis
        }
    } else {
        if speed.y > 0.0 {
            Vec2 { x: 0.0, y: 1.0 } // Collision on the positive Y axis
        } else {
            Vec2 { x: 0.0, y: -1.0 } // Collision on the negative Y axis
        }
    }
}

fn aabb_collision(a: Aabb2d, b: Aabb2d) -> bool {
    a.min.x <= b.max.x && a.max.x >= b.min.x && a.min.y <= b.max.y && a.max.y >= b.min.y
}

// pub fn move_y(amount: f32, collides_at: impl Fn(f32) -> Option<Vec2>) -> Option<(f32, Vec2)> {
//     let mut y = amount.round() as i32;

//     if y == 0 {
//         let sign = y.signum();

//         while y != 0 {
//             let collision = collides_at(y as f32);

//             if collision.is_none() {
//                 y -= sign;
//             } else {
//                 return Some((y as f32, collision.unwrap()));
//             }
//         }
//     }

//     return None;
// }
