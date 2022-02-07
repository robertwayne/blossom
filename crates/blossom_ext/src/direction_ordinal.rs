use serde::{Deserialize, Deserializer};

/// Represents a cardianl and ordinal directions, plus up or down. Directions translate to Vec3's.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Up,
    Down,
}

// We need a custom deserializer because the derive won't handle direction strings with lower or
// mixed case -- upper case only. With this impl, we can handle both cases.
impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "north" => Ok(Direction::North),
            "south" => Ok(Direction::South),
            "east" => Ok(Direction::East),
            "west" => Ok(Direction::West),
            "northeast" => Ok(Direction::NorthEast),
            "northwest" => Ok(Direction::NorthWest),
            "southeast" => Ok(Direction::SouthEast),
            "southwest" => Ok(Direction::SouthWest),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid direction: {}",
                s
            ))),
        }
    }
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::Up,
            Direction::Down,
        ]
        .iter()
        .copied()
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "north"),
            Direction::South => write!(f, "south"),
            Direction::East => write!(f, "east"),
            Direction::West => write!(f, "west"),
            Direction::NorthEast => write!(f, "northeast"),
            Direction::NorthWest => write!(f, "northwest"),
            Direction::SouthEast => write!(f, "southeast"),
            Direction::SouthWest => write!(f, "southwest"),
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
        }
    }
}
