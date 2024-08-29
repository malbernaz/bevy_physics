use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub struct Velocity {
    pub value: Vec2,
    pub remainder: Vec2,
}

impl Velocity {
    pub fn get_direction(&self) -> Vec2 {
        let mut dir = self.value;

        if dir.x != 0. {
            dir.x = dir.x.signum();
        }
        if dir.y != 0. {
            dir.y = dir.y.signum();
        }

        dir
    }

    pub fn reset_x(&mut self) {
        self.value.x = 0.;
        self.remainder.x = 0.;
    }

    pub fn reset_y(&mut self) {
        self.value.y = 0.;
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
            value: Vec2::ZERO,
            remainder: Vec2::ZERO,
        }
    }
}
