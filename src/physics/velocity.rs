use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Velocity {
    pub amount: Vec2,
    pub remainder: Vec2,
}

impl Velocity {
    pub fn get_direction(&self) -> Vec2 {
        let mut dir = self.amount;

        if dir.x != 0. {
            dir.x = dir.x.signum();
        }
        if dir.y != 0. {
            dir.y = dir.y.signum();
        }

        dir
    }

    pub fn reset_x(&mut self) {
        self.amount.x = 0.;
        self.remainder.x = 0.;
    }

    pub fn reset_y(&mut self) {
        self.amount.y = 0.;
        self.remainder.y = 0.;
    }

    pub fn reset(&mut self) {
        self.reset_x();
        self.reset_y();
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            amount: Vec2::ZERO,
            remainder: Vec2::ZERO,
        }
    }
}
