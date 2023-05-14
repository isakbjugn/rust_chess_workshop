use std::collections::HashSet;
use crate::color::Color;
use crate::piece::Piece;
use crate::square::{Square, Squares};

const KNIGHT_NAME: &str = "springar";

#[derive(Clone)]
pub struct Knight {
    color: Color,
    position: (u8, u8),
}

impl Knight {
    fn get_knight_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position.as_i8().unwrap();
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([(y + 2, x - 1), (y - 2, x - 1), (y + 2, x + 1), (y - 2, x + 1), (y - 1, x + 2), (y - 1, x - 2), (y + 1, x + 2), (y + 1, x - 2)]);
        moves.as_board_positions()
    }
}

impl Piece for Knight {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Knight {
            color,
            position,
        }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♘',
            Color::Black => '♞',
        }
    }
    fn get_name(&self) -> String {
        String::from(KNIGHT_NAME)
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> &(u8, u8) {
        &self.position
    }
    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target;
    }
    fn get_moves(&self, team: &HashSet<(u8, u8)>, _: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        self.get_knight_moves().difference(team).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::color::Color;
    use crate::piece::knight::Knight;
    use crate::piece::Piece;

    #[test]
    fn test_knight_moves_edge() {
        let knight = Knight::new(Color::White, (4, 0));
        let positions = HashSet::new();
        let legal_moves = HashSet::from_iter([(2, 1), (3, 2), (5, 2), (6, 1)]);
        assert_eq!(knight.get_moves(&positions, &positions), legal_moves)
    }

    #[test]
    fn test_knight_moves_center() {
        let knight = Knight::new(Color::White, (4, 4));
        let positions = HashSet::new();
        let legal_moves = HashSet::from_iter([(5, 2), (3, 2), (6, 3), (2, 3), (6, 5), (2, 5), (5, 6), (3, 6)]);
        assert_eq!(knight.get_moves(&positions, &positions), legal_moves)
    }
}