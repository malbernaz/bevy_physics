use bevy::{
    math::{
        bounding::{Aabb2d, IntersectsVolume},
        vec2,
    },
    prelude::*,
};
use bevy_inspector_egui::prelude::*;

use crate::physics::*;

const VELOCITY: f32 = 150.;
const ACC: f32 = 1000.;
const GRAVITY: f32 = 1000.;
const FALL_VELOCITY: f32 = -400.;
const JUMP_VELOCITY: f32 = 250.;

#[derive(Clone, Copy, Debug)]
pub struct PlayerCollider {
    pub n: RayCast,
    pub ne: RayCast,
    pub se: RayCast,
    pub s: RayCast,
    pub sw: RayCast,
    pub nw: RayCast,
}

impl PlayerCollider {
    const NORTH_OFFSET: Vec2 = vec2(0., 4.);
    const SOUTH_OFFSET: Vec2 = vec2(0., -4.);

    pub fn new() -> Self {
        Self {
            n: RayCast::new(Cardinal::North, 8.),
            ne: RayCast::new(Cardinal::East, 4.),
            se: RayCast::new(Cardinal::East, 4.),
            s: RayCast::new(Cardinal::South, 8.),
            sw: RayCast::new(Cardinal::West, 4.),
            nw: RayCast::new(Cardinal::West, 4.),
        }
    }
}

impl Shape for PlayerCollider {
    fn collides(&self, position: Vec2, aabb: &Aabb2d) -> bool {
        self.n.collides(position, aabb)
            || self.ne.collides(position + Self::NORTH_OFFSET, aabb)
            || self.se.collides(position + Self::SOUTH_OFFSET, aabb)
            || self.s.collides(position, aabb)
            || self.sw.collides(position + Self::SOUTH_OFFSET, aabb)
            || self.nw.collides(position + Self::NORTH_OFFSET, aabb)
    }

    fn get_collision_side(&self, position: Vec2, aabb: &Aabb2d) -> Option<Cardinal> {
        if self.n.ray_cast(position).intersects(aabb) {
            return Some(Cardinal::North);
        }

        if self
            .ne
            .ray_cast(position + Self::SOUTH_OFFSET)
            .intersects(aabb)
        {
            return Some(Cardinal::East);
        }

        if self
            .se
            .ray_cast(position + Self::NORTH_OFFSET)
            .intersects(aabb)
        {
            return Some(Cardinal::East);
        }

        if self.s.ray_cast(position).intersects(aabb) {
            return Some(Cardinal::South);
        }

        if self
            .sw
            .ray_cast(position + Self::SOUTH_OFFSET)
            .intersects(aabb)
        {
            return Some(Cardinal::West);
        }

        if self
            .nw
            .ray_cast(position + Self::NORTH_OFFSET)
            .intersects(aabb)
        {
            return Some(Cardinal::West);
        }

        None
    }

    fn draw_gizmo(&self, gizmos: &mut Gizmos, position: Vec2, color: Color) {
        self.n.draw_gizmo(gizmos, position, color);
        self.ne
            .draw_gizmo(gizmos, position + Self::NORTH_OFFSET, color);
        self.se
            .draw_gizmo(gizmos, position + Self::SOUTH_OFFSET, color);
        self.s.draw_gizmo(gizmos, position, color);
        self.sw
            .draw_gizmo(gizmos, position + Self::SOUTH_OFFSET, color);
        self.nw
            .draw_gizmo(gizmos, position + Self::NORTH_OFFSET, color);
    }
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    max_speed: f32,
    acceleration: f32,
    gravity: f32,
    max_fall_speed: f32,
    jump_speed: f32,
    pub grounded: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            max_speed: VELOCITY,
            acceleration: ACC,
            gravity: GRAVITY,
            max_fall_speed: FALL_VELOCITY,
            jump_speed: JUMP_VELOCITY,
            grounded: false,
        }
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    texture: Handle<Image>,
    actor: ActorBundle,
}

impl PlayerBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        let transform = Transform::from_xyz(20., 60., 0.);

        Self {
            texture,
            actor: ActorBundle::new(
                transform.translation.xy(),
                Collider::custom(PlayerCollider::new()),
            ),
            ..default()
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
        target.max(value - delta)
    } else {
        target.min(value + delta)
    }
}

pub fn handle_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Velocity, &Player)>,
) {
    if keys.pressed(KeyCode::KeyQ) {
        std::process::Command::new("clear").status().unwrap();
    }

    let Ok((mut velocity, player)) = player.get_single_mut() else {
        return;
    };

    let delta = time.delta_seconds();

    let x_axis = get_input_axis(&keys, KeyCode::ArrowRight, KeyCode::ArrowLeft);

    // let y_axis = get_input_axis(&keys, KeyCode::ArrowUp, KeyCode::ArrowDown);
    // velocity.value = Vec2::splat(75.) * Vec2::new(x_axis, y_axis);

    velocity.value.x = approach(
        velocity.value.x,
        player.max_speed * x_axis,
        player.acceleration * delta,
    );
    velocity.value.y = approach(
        velocity.value.y,
        player.max_fall_speed,
        player.gravity * delta,
    );

    if player.grounded && keys.just_pressed(KeyCode::KeyC) {
        velocity.value.y = player.jump_speed;
    }
}

pub fn handle_collision(
    mut ev_collision: EventReader<CollisionEvent>,
    mut player: Query<(Entity, &Collider, &mut Velocity, &mut Transform)>,
) {
    let Ok((entity, collider, mut velocity, mut transform)) = player.get_single_mut() else {
        return;
    };

    for ev in ev_collision.read() {
        if ev.entity.index() != entity.index() {
            continue;
        }

        let TypedShape::Custom(p) = collider.as_typed_shape() else {
            continue;
        };

        let Some(collision_side) = p.get_collision_side(
            transform.translation.xy() + ev.direction.as_vec2(),
            &ev.solid,
        ) else {
            continue;
        };

        match collision_side {
            Cardinal::North => {
                transform.translation.y = ev.solid.min.y - 8.;
                velocity.reset_y();
            }
            Cardinal::East => {
                transform.translation.x = ev.solid.min.x - 4.;
                velocity.reset_x();
            }
            Cardinal::South => {
                transform.translation.y = ev.solid.max.y + 8.;
                velocity.reset_y();
            }
            Cardinal::West => {
                transform.translation.x = ev.solid.max.x + 4.;
                velocity.reset_x();
            }
        }

        // handle one event at a time
        return;
    }
}

pub fn update_player_grounded(
    mut actor: Query<(&mut Player, &Collider, &Transform)>,
    solids: Query<(&Collider, &Transform), (With<Solid>, Without<Player>)>,
) {
    for (mut player, collider, transform) in &mut actor {
        player.grounded = solids.iter().any(|(solid, solid_transform)| {
            let TypedShape::Aabb(solid) = solid.as_typed_shape() else {
                return false;
            };

            matches!(
                collider.get_collision_side(
                    transform.translation.xy(),
                    &solid.aabb(solid_transform.translation.xy())
                ),
                Some(Cardinal::South)
            )
        });
    }
}
