use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};
use sqlx::Type;

use crate::direction::Direction;

/// Basic implementation of an integer-based 3D vector (x, y, z). This is
/// primarily used for representing the location of a game object in the world.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize, Type)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn as_vec(&self) -> Vec<i32> {
        vec![self.x, self.y, self.z]
    }
}

impl From<&[i32]> for Vec3 {
    fn from(data: &[i32]) -> Self {
        Self {
            x: data.first().copied().unwrap_or_default(),
            y: data.get(1).copied().unwrap_or_default(),
            z: data.get(2).copied().unwrap_or_default(),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<Direction> for Vec3 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => Vec3::new(0, 1, 0),
            Direction::South => Vec3::new(0, -1, 0),
            Direction::East => Vec3::new(1, 0, 0),
            Direction::West => Vec3::new(-1, 0, 0),
            Direction::Up => Vec3::new(0, 0, 1),
            Direction::Down => Vec3::new(0, 0, -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_positive() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);

        let c = a + b;

        assert_eq!(c.x, 5);
        assert_eq!(c.y, 7);
        assert_eq!(c.z, 9);
    }

    #[test]
    fn add_negative() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(-4, -5, -6);

        let c = a + b;

        assert_eq!(c.x, -3);
        assert_eq!(c.y, -3);
        assert_eq!(c.z, -3);
    }

    #[test]
    fn compare() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(1, 2, 3);

        assert_eq!(a, b);
        assert_ne!(a, Vec3::new(-1, -2, -3));
    }

    #[test]
    fn subtract() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);

        let c = a - b;

        assert_eq!(c.x, -3);
        assert_eq!(c.y, -3);
        assert_eq!(c.z, -3);
    }

    #[test]
    fn from_direction() {
        let north = Vec3::from(Direction::North);
        let south = Vec3::from(Direction::South);
        let up = Vec3::from(Direction::Up);

        assert_eq!(north, Vec3::new(0, 1, 0));
        assert_eq!(south, Vec3::new(0, -1, 0));
        assert_eq!(up, Vec3::new(0, 0, 1));
    }
}
