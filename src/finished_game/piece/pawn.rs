use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{Square, Squares};

#[derive(Clone)]
pub struct Pawn {
    color: Color,
    position: (u8, u8),
}

impl Pawn {
    pub fn get_forward_moves(&self, other_pieces: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position;
        match (self.color, y) {
            (Color::White, 1) if other_pieces.contains(&(x, y + 1)) => HashSet::new(),
            (Color::White, 1) => HashSet::from_iter([(x, 2), (x, 3)]),
            (Color::White, _) => HashSet::from_iter([(x, y + 1)]),
            (Color::Black, 6) if other_pieces.contains(&(x, y - 1)) => HashSet::new(),
            (Color::Black, 6) => HashSet::from_iter([(x, 5), (x, 4)]),
            (Color::Black, _) => HashSet::from_iter([(x, y - 1)]),
        }
        .difference(other_pieces)
        .cloned()
        .collect()
    }

    pub fn get_capture_moves(&self, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        match self.color {
            Color::White => HashSet::from_iter([(x - 1, y + 1), (x + 1, y + 1)]),
            Color::Black => HashSet::from_iter([(x - 1, y - 1), (x + 1, y - 1)]),
        }
        .as_board_positions()
        .intersection(rival_team)
        .cloned()
        .collect()
    }
}

impl Piece for Pawn {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Pawn { color, position }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♟',
            Color::Black => '♙',
        }
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
        let all_pieces: HashSet<_> = team.union(rival_team).cloned().collect();
        let moves = self.get_forward_moves(&all_pieces);
        let capture_moves = self.get_capture_moves(rival_team);
        moves.union(&capture_moves).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::finished_game::color::Color;
    use crate::finished_game::piece::pawn::Pawn;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};
    use crate::{assert_eq_set, empty_set, set};

    #[test]
    fn two_moves_for_e2_opening_move() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = set!["e3", "e4"];
        assert_eq_set!(pawn.get_forward_moves(&empty_set!()), legal_moves)
    }

    #[test]
    fn two_capture_moves_for_e2_opening_move() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = set!["d3", "f3"];
        let rival_team = set!["d3", "e3", "f3", "g3"];
        assert_eq_set!(pawn.get_capture_moves(&rival_team), legal_moves)
    }

    #[test]
    fn no_opening_moves_for_blocked_pawn() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        assert_eq_set!(empty_set!(), pawn.get_moves(&set!["e2"], &set!["e3"]))
    }

    #[test]
    fn one_opening_move_for_half_blocked_pawn() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        assert_eq_set!(&set!["e3"], pawn.get_moves(&set!["e2"], &set!["e4"]))
    }
}
