use std::collections::HashSet;

pub trait Squares {
    fn as_board_positions(&self) -> HashSet<(u8, u8)>;
}

impl Squares for HashSet<(i8, i8)> {
    fn as_board_positions(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .filter(|(y, x)| (0..8).contains(y) && (0..8).contains(x))
            .map(|(y, x)| (y as u8, x as u8))
            .collect()
    }
}

impl Squares for HashSet<(u8, u8)> {
    fn as_board_positions(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .filter(|(y, x)| (0..8).contains(y) && (0..8).contains(x))
            .collect()
    }
}

impl Squares for [&str] {
    fn as_board_positions(&self) -> HashSet<(u8, u8)> {
        self.iter().cloned()
            .map(|s| s.as_u8().unwrap())
            .collect()
    }
}

pub trait Square {
    fn as_i8(&self) -> Option<(i8, i8)>;
    fn as_u8(&self) -> Option<(u8, u8)>;
}

impl Square for (u8, u8) {
    fn as_i8(&self) -> Option<(i8, i8)> {
        Some( (self.0 as i8, self.1 as i8) )
    }

    fn as_u8(&self) -> Option<(u8, u8)> {
        Some(*self)
    }
}

impl Square for &str {
    fn as_i8(&self) -> Option<(i8, i8)> {
        self.as_u8().map(|coordinate| (coordinate.0 as i8, coordinate.1 as i8))
    }

    fn as_u8(&self) -> Option<(u8, u8)> {
        if self.chars().count() != 2 { return None }
        let mut chars = self.chars();
        let col = chars.next().unwrap().to_ascii_lowercase() as u8 - 97;
        let row = chars.next().unwrap().to_digit(10).unwrap() as u8 - 1;

        if col < 8 && row < 8 {
            return Some((row, col));
        }
        None
    }
}

pub trait MoveDirections {
    /// Filter moves which are blocked by other pieces, ensuring pieces such as the rook cannot move
    /// through other pieces
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
