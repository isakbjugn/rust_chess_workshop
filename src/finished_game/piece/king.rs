use std::collections::HashSet;
use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{Square, Squares};

pub const KING_NAME: &str = "konge";

#[derive(Clone)]
pub struct King {
    pub color: Color,
    pub position: (u8, u8),
}

impl King {
    fn get_king_moves(&self) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([(x - 1, y + 1), (x, y + 1), (x + 1, y + 1), (x - 1, y), (x + 1, y), (x - 1, y - 1), (x, y - 1), (x + 1, y - 1)]);
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
    use crate::{assert_eq_set, empty_set, set};
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::king::King;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};

    #[test]
    fn test_king_moves_edge() {
        let king = King::new(Color::White, "e1".as_u8().unwrap());
        let legal_moves = set!["d1", "d2", "e2", "f2", "f1"];
        assert_eq_set!(king.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }

    #[test]
    fn test_king_moves_center() {
        let king = King::new(Color::White, "e5".as_u8().unwrap());
        let legal_moves = set!["d4", "d5", "d6", "e6", "f6", "f5", "f4", "e4"];
        assert_eq_set!(king.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }
}