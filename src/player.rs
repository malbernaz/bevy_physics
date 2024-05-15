use bevy::prelude::*;

use crate::physics::{actor::*, collider::*, solid::*};

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
    pub fn new(texture: Handle<Image>) -> Self {
        let sprite = SpriteBundle {
            texture,
            ..default()
        };

        Self {
            sprite,
            name: Name::new("Player"),
            player: Player::default(),
            actor: ActorBundle::new(Collider::new(8., 16.)),
        }
    }
}

pub fn get_input_axis(input: &ButtonInput<KeyCode>, positive: KeyCode, negative: KeyCode) -> f32 {
    let positive = if input.pressed(positive) { 1. } else { 0. };
    let negative = if input.pressed(negative) { 1. } else { 0. };
    positive - negative
}

pub fn approach(from: f32, to: f32, delta: f32) -> f32 {
    if from < to {
        return to.min(from + delta);
    }

    to.max(from - delta)
}

const SPEED: f32 = 300.;
const ACC: f32 = 10.;
const GRAVITY: f32 = 9.;
const FALL_SPEED: f32 = -1000.;

pub fn movement(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut player: Query<&mut Player>) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let axis = get_input_axis(&keys, KeyCode::ArrowRight, KeyCode::ArrowLeft);

    let delta = time.delta_seconds();

    player.speed.x = approach(player.speed.x, SPEED * axis * delta, ACC * delta);
    player.speed.y = approach(player.speed.y, FALL_SPEED * delta, GRAVITY * delta);

    if keys.pressed(KeyCode::KeyC) {
        player.speed.y = 300. * delta;
    }
}

pub fn collision_system(
    mut player: Query<(&mut Transform, &mut Player, &Collider), With<Player>>,
    mut solids: Query<(&Transform, &Collider), (With<Solid>, Without<Player>)>,
) {
    let Ok((mut p_transform, mut player, p_collider)) = player.get_single_mut() else {
        return;
    };

    let mut amount = player.speed.round();

    for (s_transform, s_collider) in &mut solids {
        let p_pos = p_transform.translation.xy();
        let s_pos = s_transform.translation.xy();

        if let Some(col) = collide(
            p_pos + amount,
            p_collider.rect.half_size,
            s_pos,
            s_collider.rect.half_size,
        ) {
            println!("{:?}", col);
            // amount.x = 0.;
            player.speed.x = 0.;
            amount.y = (col.y - p_collider.rect.half_size.y) * amount.y.signum();
            player.speed.y = 0.;
            // p_transform.translation.y += (col.y - p_collider.rect.half_size.y) * amount.y.signum();
        } else {
            p_transform.translation.x += amount.x;
            // println!("{:?}", p_transform.translation);
        }

        // if let Some(collision) = collide(
        //     Vec2::new(0., p_pos.x + amount.x),
        //     p_collider.rect.half_size,
        //     s_pos,
        //     s_collider.rect.half_size,
        // ) {
        //     // if collision.y != 0. {
        //     //     amount.y = 0.;
        //     // }
        // } else {
        //     p_transform.translation.y += amount.y;
        // }

        // if let Some((y, _collision)) = move_y(amount.y, |sign| {
        //     collide(
        //         Vec2::new(0., p_pos.y + sign),
        //         p_collider.rect.half_size,
        //         s_pos,
        //         s_collider.rect.half_size,
        //     )
        // }) {
        //     println!("{:?} {:?}", y, amount.y);
        //     amount.y = 0.;
        //     player.speed.y = 0.;
        // } else {
        //     p_transform.translation.y += amount.y;
        // }
    }
    // println!("{:?}", amount.y);
    p_transform.translation.y += amount.y;
}

pub fn move_y(amount: f32, collides_at: impl Fn(f32) -> Option<Vec2>) -> Option<(f32, Vec2)> {
    let mut y = amount.round() as i32;

    if y == 0 {
        let sign = y.signum();

        while y != 0 {
            let collision = collides_at(y as f32);

            if collision.is_none() {
                y -= sign;
            } else {
                return Some((y as f32, collision.unwrap()));
            }
        }
    }

    return None;
}

struct Hit {
    pub direction: Vec2,
}

impl Default for Hit {
    fn default() -> Self {
        Self {
            direction: Vec2::ZERO,
        }
    }
}

impl Hit {
    pub fn new(direction: Vec2) -> Self {
        Self { direction }
    }

    pub fn is_hit_left(&self) -> bool {
        self.direction.x < 0.
    }

    pub fn is_hit_right(&self) -> bool {
        self.direction.x > 0.
    }

    pub fn is_hit_up(&self) -> bool {
        self.direction.y < 0.
    }

    pub fn is_hit_down(&self) -> bool {
        self.direction.y > 0.
    }
}

pub fn collide(a_pos: Vec2, a_half_size: Vec2, s_pos: Vec2, s_half_size: Vec2) -> Option<Vec2> {
    let actor_min = a_pos - a_half_size;
    let actor_max = a_pos + a_half_size;
    let solid_min = s_pos - s_half_size;
    let solid_max = s_pos + s_half_size;

    if actor_min.x < solid_max.x
        && actor_max.x > solid_min.x
        && actor_min.y < solid_max.y
        && actor_max.y > solid_min.y
    {
        // let overlap = Vec2::new(
        //     (actor_max.x - solid_min.x).min(solid_max.x - actor_min.x),
        //     (actor_max.y - solid_min.y).min(solid_max.y - actor_min.y),
        // );

        let hit = Hit::new(Vec2::new(
            (actor_max.x - solid_min.x).min(solid_max.x - actor_min.x),
            (actor_max.y - solid_min.y).min(solid_max.y - actor_min.y),
        ));

        return Some(hit.direction);
    }

    return None;
}
