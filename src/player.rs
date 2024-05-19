use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::physics::*;

const VELOCITY: f32 = 150.;
const ACC: f32 = 10.;
const GRAVITY: f32 = 10.;
const FALL_VELOCITY: f32 = -400.;
const JUMP_VELOCITY: f32 = 300.;

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    max_speed: f32,
    acceleration: f32,
    gravity: f32,
    max_fall_speed: f32,
    jump_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            max_speed: VELOCITY,
            acceleration: ACC,
            gravity: GRAVITY,
            max_fall_speed: FALL_VELOCITY,
            jump_speed: JUMP_VELOCITY,
        }
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    texture: Handle<Image>,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,
    actor: ActorBundle,
}

impl PlayerBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        let transform = Transform::from_xyz(20., 56., 0.);

        Self {
            texture,
            actor: ActorBundle::new(transform.translation.xy(), Vec2::new(8. / 2., 16. / 2.)),
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

pub fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut player: Query<(&mut Velocity, &Player)>) {
    let Ok((mut velocity, player)) = player.get_single_mut() else {
        return;
    };

    let x_axis = get_input_axis(&keys, KeyCode::ArrowRight, KeyCode::ArrowLeft);

    velocity.amount.x = approach(
        velocity.amount.x,
        player.max_speed * x_axis,
        player.acceleration,
    );
    velocity.amount.y = approach(velocity.amount.y, player.max_fall_speed, player.gravity);

    if keys.just_pressed(KeyCode::KeyC) {
        velocity.amount.y = player.jump_speed;
    }
}

pub fn handle_collision(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_collision: EventReader<CollisionEvent>,
    mut player: Query<(&mut Velocity, Entity), With<Player>>,
) {
    let Ok((mut velocity, entity)) = player.get_single_mut() else {
        return;
    };

    for CollisionEvent {
        entity: actor_entity,
        collision_type,
    } in ev_collision.read()
    {
        if actor_entity.index() == entity.index() {
            match collision_type {
                CollisionType::Horizontal => {
                    velocity.reset_x();
                }
                CollisionType::Vertical => {
                    if !keys.just_pressed(KeyCode::KeyC) {
                        velocity.reset_y();
                    }
                }
            };
        }
    }
}
