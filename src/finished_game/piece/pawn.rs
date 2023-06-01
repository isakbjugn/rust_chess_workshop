use std::collections::HashSet;
use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{Square, Squares};

const PAWN_NAME: &str = "bonde";

#[derive(Clone)]
pub struct Pawn {
    color: Color,
    position: (u8, u8),
}

impl Pawn {
    pub fn get_pawn_moves(&self) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        let moves: HashSet<(i8, i8)> = match self.color {
            Color::White if y == 1 => HashSet::from_iter([(x, 2), (x, 3)]),
            Color::White => HashSet::from_iter([(x, y + 1)]),
            Color::Black if y == 6 => HashSet::from_iter([(x, 5), (x, 4)]),
            Color::Black => HashSet::from_iter([(x, y - 1)]),
        };
        moves.as_board_positions()
    }

    pub fn get_pawn_capture_moves(&self) -> HashSet<(u8, u8)> {
        // TODO: Add possible en passant captures
        let (x, y) = self.position.as_i8().unwrap();
        let capture_moves: HashSet<(i8, i8)> = match self.color {
            Color::White => HashSet::from_iter([(x - 1, y + 1), (x + 1, y + 1)]),
            Color::Black => HashSet::from_iter([(x - 1, y - 1), (x + 1, y - 1)]),
        };
        capture_moves.as_board_positions()
    }
}

impl Piece for Pawn {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Pawn {
            color,
            position,
        }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♙',
            Color::Black => '♟',
        }
    }
    fn get_name(&self) -> String {
        String::from(PAWN_NAME)
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
        let all_pieces = team.union(rival_team).cloned().collect();
        let moves: HashSet<(u8, u8)> = self.get_pawn_moves().difference(&all_pieces).cloned().collect();
        let capture_moves: HashSet<(u8, u8)> = self.get_pawn_capture_moves().intersection(rival_team).cloned().collect();
        moves.union(&capture_moves).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::assert_eq_set;
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::pawn::Pawn;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};

    #[test]
    fn two_moves_for_e2_opening_move() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = ["e3", "e4"].as_board_positions();
        assert_eq_set!(pawn.get_pawn_moves(), legal_moves)
    }

    #[test]
    fn two_capture_moves_for_e2_opening_move() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = ["d3", "f3"].as_board_positions();
        assert_eq_set!(pawn.get_pawn_capture_moves(), legal_moves)
    }

    #[test]
    fn no_opening_moves_for_blocked_pawn() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = HashSet::new();
        assert_eq_set!(
            legal_moves,
            pawn.get_moves(
                &["e2"].as_board_positions(),
                &["e3"].as_board_positions(),
            ))
    }
}