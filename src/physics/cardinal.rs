use bevy::{math::InvalidDirectionError, prelude::*};

#[derive(Debug, Copy, Clone, Reflect, PartialEq, Eq)]
pub enum Cardinal {
    West,
    North,
    East,
    South,
}

impl Cardinal {
    pub fn as_vec2(&self) -> Vec2 {
        match self {
            Cardinal::West => Vec2::NEG_X,
            Cardinal::North => Vec2::Y,
            Cardinal::East => Vec2::X,
            Cardinal::South => Vec2::NEG_Y,
        }
    }

    pub fn x(&self) -> f32 {
        match self {
            Cardinal::West => -1.,
            Cardinal::North => 0.,
            Cardinal::East => 1.,
            Cardinal::South => 0.,
        }
    }

    pub fn y(&self) -> f32 {
        match self {
            Cardinal::West => 0.,
            Cardinal::North => 1.,
            Cardinal::East => 0.,
            Cardinal::South => -1.,
        }
    }

    pub fn isHorizontal(&self) -> bool {
        *self == Self::West || *self == Self::East
    }

    pub fn isVertical(&self) -> bool {
        *self == Self::North || *self == Self::South
    }

    pub fn from_vec2(vec: Vec2) -> Result<Self, ()> {
        if vec.x < 0. {
            Ok(Cardinal::West)
        } else if vec.x > 0. {
            Ok(Cardinal::East)
        } else if vec.y > 0. {
            Ok(Cardinal::North)
        } else if vec.y < 0. {
            Ok(Cardinal::South)
        } else {
            Err(())
        }
    }

    pub fn as_dir(&self) -> Result<Dir2, InvalidDirectionError> {
        Dir2::new(self.as_vec2())
    }
}
