use std::collections::HashSet;
use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{Square, Squares};

#[derive(Clone)]
pub struct Knight {
    color: Color,
    position: (u8, u8),
}

const KNIGHT_NAME: &str = "springar";

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
        let (x, y) = self.position.as_i8().unwrap();
        HashSet::from_iter([
                            (x - 1, y + 2), (x + 1, y + 2),
            (x - 2, y + 1),                                 (x + 2, y + 1),

            (x - 2, y - 1),                                 (x + 2, y - 1),
                            (x - 1, y - 2), (x + 1, y - 2),
        ]).as_board_positions().difference(team).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{assert_eq_set, empty_set, set};
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::knight::Knight;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};

    #[test]
    fn test_knight_moves_edge() {
        let knight = Knight::new(Color::White, "a5".as_u8().unwrap());
        let legal_moves = set!["b3", "c4", "c6", "b7"];
        assert_eq_set!(knight.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }

    #[test]
    fn test_knight_moves_center() {
        let knight = Knight::new(Color::White, "e5".as_u8().unwrap());
        let legal_moves = set!["c6", "c4", "d7", "d3", "f7", "f3", "g6", "g4"];
        assert_eq_set!(knight.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }
}