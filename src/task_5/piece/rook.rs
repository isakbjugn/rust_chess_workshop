use std::collections::HashSet;
use crate::color::Color;
use crate::task_5::piece::Piece;

const ROOK_NAME: &str = "tårn";

#[derive(Clone)]
pub struct Rook {
    pub color: Color,
    pub position: (u8, u8),
}

impl Piece for Rook {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Rook {
            color,
            position,
        }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♖',
            Color::Black => '♜',
        }
    }
    fn get_name(&self) -> String {
        String::from(ROOK_NAME)
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
    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{assert_eq_set, empty_set, set};
    use crate::color::Color;
    use crate::task_5::piece::rook::Rook;
    use crate::task_5::piece::Piece;
    use crate::square::{Square, Squares};

    #[test]
    fn test_queen_moves_1() {
        let rook = Rook::new(Color::White, "a1".as_u8().unwrap());
        let legal_moves = set![
            "a2", "a3", "a4", "a5", "a6", "a7", "a8",
            "b1", "c1", "d1", "e1", "f1", "g1", "h1",
        ];
        assert_eq_set!(rook.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }

    #[test]
    fn test_queen_moves_2() {
        let rook = Rook::new(Color::White, "d4".as_u8().unwrap());
        let legal_moves = set![
            "c4", "b4", "a4",
            "d5", "d6", "d7", "d8",
            "e4", "f4", "g4", "h4",
            "d3", "d2",  "d1",
        ];
        assert_eq_set!(rook.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }
}