use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use crate::delta::{Delta, CARDINAL_DELTAS, ORDINAL_DELTAS};

#[typeshare]
/// Represents the location of something on a 2D grid
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Combine with a Delta to get a new position relative to the old position
    pub fn add_delta(&self, delta: &Delta) -> Position {
        let new_x = self.x + delta.x as i32;
        let new_y = self.y + delta.y as i32;

        Position { x: new_x, y: new_y }
    }

    /// Get a vector of all four positions surrounding a position on the grid
    pub fn cardinal_positions(&self) -> Vec<Position> {
        CARDINAL_DELTAS
            .iter()
            .map(|delta| self.add_delta(delta))
            .collect()
    }

    /// Get a vector of all four ordinal surrounding a position on the grid
    pub fn ordinal_positions(&self) -> Vec<Position> {
        ORDINAL_DELTAS
            .iter()
            .map(|delta| self.add_delta(delta))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use ae_direction::{Cardinal, Direction, Ordinal};

    use super::*;

    #[test]
    fn add_delta() {
        let pos = Position { x: 1, y: 1 };
        assert_eq!(pos, Position { x: 1, y: 1 });

        let pos = pos.add_delta(&Delta::from(Direction::Cardinal(Cardinal::North)));
        assert_eq!(pos, Position { x: 1, y: 0 });

        let pos = pos.add_delta(&Delta::from(Direction::Cardinal(Cardinal::East)));
        assert_eq!(pos, Position { x: 2, y: 0 });

        let pos = pos.add_delta(&Delta::from(Direction::Ordinal(Ordinal::Southwest)));
        assert_eq!(pos, Position { x: 1, y: 1 });
    }

    #[test]
    fn cardinal_positions() {
        let pos = Position { x: 5, y: 5 };

        let cardinal_positions = pos.cardinal_positions();

        assert!(cardinal_positions.contains(&Position { x: 5, y: 4 }));
        assert!(cardinal_positions.contains(&Position { x: 5, y: 6 }));
        assert!(cardinal_positions.contains(&Position { x: 4, y: 5 }));
        assert!(cardinal_positions.contains(&Position { x: 6, y: 5 }));
    }

    #[test]
    fn ordinal_positions() {
        let pos = Position { x: 5, y: 5 };

        let cardinal_positions = pos.ordinal_positions();

        assert!(cardinal_positions.contains(&Position { x: 6, y: 4 }));
        assert!(cardinal_positions.contains(&Position { x: 6, y: 6 }));
        assert!(cardinal_positions.contains(&Position { x: 4, y: 4 }));
        assert!(cardinal_positions.contains(&Position { x: 4, y: 6 }));
    }
}
