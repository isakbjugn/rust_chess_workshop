use std::collections::HashSet;
use crate::utils::square_name_to_coordinate;

pub trait Squares {
    fn as_board_position(&self) -> HashSet<(u8, u8)>;
}

impl Squares for HashSet<(i8, i8)> {
    fn as_board_position(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .filter(|(y, x)| (0..8).contains(y) && (0..8).contains(x))
            .map(|(y, x)| (y as u8, x as u8))
            .collect()
    }
}

impl Squares for HashSet<(u8, u8)> {
    fn as_board_position(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .filter(|(y, x)| (0..8).contains(y) && (0..8).contains(x))
            .collect()
    }
}

impl Squares for [&str] {
    fn as_board_position(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .map(|s| square_name_to_coordinate(s).unwrap())
            .collect()
    }
}

pub trait Square {
    fn as_i8(&self) -> (i8, i8);
    fn as_u8(&self) -> (u8, u8);
}

impl Square for (u8, u8) {
    fn as_i8(&self) -> (i8, i8) {
        (self.0 as i8, self.1 as i8)
    }

    fn as_u8(&self) -> (u8, u8) {
        *self
    }
}

impl Square for &str {
    fn as_i8(&self) -> (i8, i8) {
        let coordinate = self.as_u8();
        (coordinate.0 as i8, coordinate.1 as i8)
    }

    fn as_u8(&self) -> (u8, u8) {
        square_name_to_coordinate(self).unwrap()
    }
}

pub trait MoveDirections {
    fn filter_move_directions(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)>;
}

impl MoveDirections for HashSet<Vec<(u8, u8)>> {
    fn filter_move_directions(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let mut moves = HashSet::new();
        for line in self {
            'direction: for square in line {
                if team.contains(square) {
                    break 'direction
                }
                moves.insert(*square);
                if rival_team.contains(square) {
                    break 'direction
                }
            }
        }
        moves
    }
}
