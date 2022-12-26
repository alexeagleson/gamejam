use std::fmt::Display;

use rand::{distributions::Standard, prelude::Distribution};

#[derive(PartialEq, Clone, Debug)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Display for Cardinal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cardinal::North => write!(f, "north"),
            Cardinal::East => write!(f, "east"),
            Cardinal::South => write!(f, "south"),
            Cardinal::West => write!(f, "west"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Ordinal {
    Northeast,
    Southeast,
    Southwest,
    Northwest,
}

impl Display for Ordinal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ordinal::Northeast => write!(f, "northeast"),
            Ordinal::Southeast => write!(f, "southeast"),
            Ordinal::Southwest => write!(f, "southwest"),
            Ordinal::Northwest => write!(f, "northwest"),
        }
    }
}

/// Represents a direction, either cardinal or ordinal
#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Cardinal(Cardinal),
    Ordinal(Ordinal),
}

impl Distribution<Cardinal> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Cardinal {
        match rng.gen_range(0..=3) {
            0 => Cardinal::North,
            1 => Cardinal::East,
            2 => Cardinal::South,
            3 => Cardinal::West,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    /// Returns the opposite (180 degrees) direction of a `Direction`
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Cardinal(cardinal) => match cardinal {
                Cardinal::North => Direction::Cardinal(Cardinal::South),
                Cardinal::East => Direction::Cardinal(Cardinal::West),
                Cardinal::South => Direction::Cardinal(Cardinal::North),
                Cardinal::West => Direction::Cardinal(Cardinal::East),
            },
            Direction::Ordinal(ordinal) => match ordinal {
                Ordinal::Northeast => Direction::Ordinal(Ordinal::Southwest),
                Ordinal::Southeast => Direction::Ordinal(Ordinal::Northwest),
                Ordinal::Southwest => Direction::Ordinal(Ordinal::Northeast),
                Ordinal::Northwest => Direction::Ordinal(Ordinal::Southeast),
            },
        }
    }

    /// Returns the clockwise (90 degrees) direction of a `Direction`
    pub fn clockwise(&self) -> Direction {
        match self {
            Direction::Cardinal(cardinal) => match cardinal {
                Cardinal::North => Direction::Cardinal(Cardinal::East),
                Cardinal::East => Direction::Cardinal(Cardinal::South),
                Cardinal::South => Direction::Cardinal(Cardinal::West),
                Cardinal::West => Direction::Cardinal(Cardinal::North),
            },
            Direction::Ordinal(ordinal) => match ordinal {
                Ordinal::Northeast => Direction::Ordinal(Ordinal::Southeast),
                Ordinal::Southeast => Direction::Ordinal(Ordinal::Southwest),
                Ordinal::Southwest => Direction::Ordinal(Ordinal::Northwest),
                Ordinal::Northwest => Direction::Ordinal(Ordinal::Northeast),
            },
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Cardinal(cardinal) => write!(f, "{}", cardinal),
            Direction::Ordinal(ordinal) => write!(f, "{}", ordinal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clockwise() {
        let dir = Direction::Cardinal(Cardinal::North);
        assert_eq!(dir.clockwise(), Direction::Cardinal(Cardinal::East));

        let dir = Direction::Ordinal(Ordinal::Southwest);
        assert_eq!(dir.clockwise(), Direction::Ordinal(Ordinal::Northwest));
    }

    #[test]
    fn reverse() {
        let dir = Direction::Cardinal(Cardinal::North);
        assert_eq!(dir.reverse(), Direction::Cardinal(Cardinal::South));

        let dir = Direction::Ordinal(Ordinal::Southwest);
        assert_eq!(dir.reverse(), Direction::Ordinal(Ordinal::Northeast));
    }
}
