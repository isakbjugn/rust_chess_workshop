use std::collections::HashSet;
use crate::color::Color;
use crate::piece::Piece;
use crate::square::{Square, Squares};

pub const KING_NAME: &str = "konge";

#[derive(Clone)]
pub struct King {
    pub color: Color,
    pub position: (u8, u8),
}

impl King {
    fn get_king_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position.as_i8().unwrap();
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([(y + 1, x - 1), (y + 1, x), (y + 1, x + 1), (y, x - 1), (y, x + 1), (y - 1, x - 1), (y - 1, x), (y - 1, x + 1)]);
        moves.as_board_positions()
    }
}

impl Piece for King {
    fn new(color: Color, position: (u8, u8)) -> Self {
        King {
            color,
            position,
        }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♔',
            Color::Black => '♚',
        }
    }

    fn get_name(&self) -> String {
        String::from(KING_NAME)
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
        self.get_king_moves().difference(team).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::color::Color;
    use crate::piece::king::King;
    use crate::piece::Piece;

    #[test]
    fn test_king_moves_edge() {
        let king = King::new(Color::White, (0, 5));
        let positions = HashSet::new();
        let legal_moves = HashSet::from_iter([(0, 4), (1, 4), (1, 5), (1, 6), (0, 6)]);
        assert_eq!(king.get_moves(&positions, &positions), legal_moves)
    }

    #[test]
    fn test_king_moves_center() {
        let king = King::new(Color::White, (4, 4));
        let positions = HashSet::new();
        let legal_moves = HashSet::from_iter([(5, 3), (5, 4), (5, 5), (4, 3), (4, 5), (3, 3), (3, 4), (3, 5)]);
        assert_eq!(king.get_moves(&positions, &positions), legal_moves)
    }
}