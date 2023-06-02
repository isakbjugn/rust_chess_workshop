use std::collections::HashSet;
use crate::square::{Square, Squares};
use crate::color::Color;
use crate::task_3::piece::Piece;

const KNIGHT_NAME: &str = "springar";

#[derive(Clone)]
pub struct Knight {
    position: (u8, u8),
    color: Color,
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

    fn get_moves(&self, _team: &HashSet<(u8, u8)>, _rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([(x - 1, y + 2), (x - 1, y - 2), (x + 1, y + 2), (x + 1, y - 2), (x + 2, y - 1), (x - 2, y - 1), (x + 2, y + 1), (x - 2, y + 1)]);
        moves.as_board_positions()
        // TODO remove implementation
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{assert_eq_set, empty_set, set};
    use crate::square::{Square, Squares};
    use crate::color::Color;
    use crate::task_3::piece::knight::Knight;
    use crate::task_3::piece::Piece;

    #[test]
    fn knight_moves_in_start_position() {
        let knight = Knight::new(Color::White, "b1".as_u8().unwrap());
        let legal_moves = set!["a3", "c3", "d2"];
        assert_eq_set!(legal_moves, knight.get_moves(&HashSet::from([knight.position]), &empty_set!()));
    }

    #[test]
    fn knight_moves_in_center() {
        let knight = Knight::new(Color::White, "d5".as_u8().unwrap());
        let legal_moves = set!["b4", "c3", "e3", "f4", "c7", "b6", "e7", "f6"];
        assert_eq_set!(legal_moves, knight.get_moves(&HashSet::from([knight.position]), &empty_set!()));
    }
}
