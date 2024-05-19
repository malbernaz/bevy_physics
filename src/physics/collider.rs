use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use super::*;

#[derive(Component, Reflect, Default, InspectorOptions, Clone, Copy)]
#[reflect(Component, InspectorOptions)]
pub struct Collider {
    pub min: Vec2,
    pub max: Vec2,
}

impl Collider {
    pub fn new(center: Vec2, half_size: Vec2) -> Self {
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    /// gets collider center
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) / 2.
    }

    /// gets collider half size
    pub fn half_size(&self) -> Vec2 {
        (self.max - self.min) / 2.
    }

    /// checks for collisions with other collider
    pub fn collides_with(&self, other: &Self) -> bool {
        let x = self.min.x < other.max.x && self.max.x > other.min.x;
        let y = self.min.y < other.max.y && self.max.y > other.min.y;

        x && y
    }

    /// checks if a collision will occur with another collider by moving a given amount
    pub fn collides_at(&self, amount: Vec2, other: &Self) -> bool {
        let center = self.center();
        let half_size = self.half_size();
        let moved = Self::new(center + amount, half_size);

        moved.collides_with(other)
    }

    /// gets the minimal distance between the sides of the rect
    pub fn min_diff(&self, other: &Self) -> Vec2 {
        let u_diff = (self.max.y - other.min.y).abs();
        let r_diff = (self.max.x - other.min.x).abs();
        let d_diff = (self.min.y - other.max.y).abs();
        let l_diff = (self.min.x - other.max.x).abs();

        Vec2::new(r_diff.min(l_diff), u_diff.min(d_diff))
    }

    /// update position by an absolute amount
    pub fn update_position(&mut self, center: Vec2) {
        let half_size = self.half_size();
        *self = Self::new(center, half_size);
    }

    /// update position by a relative amount
    pub fn update_position_by(&mut self, amount: Vec2) {
        let center = self.center();
        self.update_position(center + amount);
    }
}

pub enum CollisionType {
    Horizontal,
    Vertical,
}

#[derive(Event)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collision_type: CollisionType,
}

pub fn draw_collider_gizmos(mut gizmos: Gizmos, query: Query<(&Collider, Option<&Actor>)>) {
    for (collider, actor) in &query {
        let color = if actor.is_some() {
            Color::rgb(0., 0., 1.)
        } else {
            Color::rgb(0., 1., 0.)
        };

        gizmos.primitive_2d(
            Rectangle::from_corners(collider.min, collider.max),
            collider.center(),
            0.,
            color,
        );
    }
}
