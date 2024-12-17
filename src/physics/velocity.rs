use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
pub struct Velocity {
    pub value: Vec2,
    pub remainder: Vec2,
}

impl Velocity {
    // returns sign and removes negative sign from 0
    pub fn get_direction(&self) -> Vec2 {
        let mut dir = self.value;

        dir.x = if dir.x == -0. { 0. } else { dir.x.signum() };
        dir.y = if dir.y == -0. { 0. } else { dir.y.signum() };

        dir
    }

    #[inline]
    pub fn reset_x(&mut self) {
        self.value.x = 0.;
        self.remainder.x = 0.;
    }

    #[inline]
    pub fn reset_y(&mut self) {
        self.value.y = 0.;
        self.remainder.y = 0.;
    }
}
