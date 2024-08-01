use std::collections::HashSet;
use crate::empty_set;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{Square, Squares};

#[derive(Clone)]
pub struct Pawn {
    color: Color,
    position: (u8, u8),
}

impl Pawn {
    // Burde vi allerede fra oppgave 1 skilt disse metodene i to, slik at det ble enklere for
    // deltakere å avgrense hva de skal implementere?
    fn get_forward_moves(&self, other_pieces: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position;
        match (self.color, y) {
            (Color::White, _) if other_pieces.contains(&(x, y + 1)) => empty_set!(),
            (Color::White, 1) if other_pieces.contains(&(x, 3)) => HashSet::from_iter([(x, 2)]),
            (Color::White, 1) => HashSet::from_iter([(x, 2), (x, 3)]),
            (Color::White, _) => HashSet::from_iter([(x, y + 1)]),
            (Color::Black, _) if other_pieces.contains(&(x, y - 1)) => empty_set!(),
            (Color::Black, 6) if other_pieces.contains(&(x, 4)) => HashSet::from_iter([(x, 5)]),
            (Color::Black, 6) => HashSet::from_iter([(x, 5), (x, 4)]),
            (Color::Black, _) => HashSet::from_iter([(x, y - 1)]),
        }
    }
    fn get_capture_moves(&self, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        match self.color {
            Color::White => HashSet::from_iter([(x - 1, y + 1), (x + 1, y + 1)]),
            Color::Black => HashSet::from_iter([(x - 1, y - 1), (x + 1, y - 1)]),
        }.as_board_positions().intersection(rival_team).cloned().collect()
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

    /// Flytter brikken til posisjonen indikert av `target` ved å mutere sin egen posisjon
    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target;
    }

    /// Returnerer et HashSet som inneholder gyldige posisjoner bonden kan flytte til. En posisjon
    /// defineres av et to-tuppel med koordinater, der f.eks (0, 1) korresponderer til feltet A2.
    /// `square.rs` inneholder hjelpefunksjoner for å konvertere f.eks `"a2"` til `(0, 1)` og omvendt.
    ///
    /// # Argumenter
    /// - `team` Referanse til et HashSet som inneholder dine brikkers posisjoner.
    /// - `rival_team` Referanse til et HashSet som inneholder posisjonene til motstanderens brikker.
    ///
    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let all_pieces = team.union(rival_team).cloned().collect();
        let forward_moves = self.get_forward_moves(&all_pieces);
        let capture_moves = self.get_capture_moves(rival_team);
        forward_moves.union(&capture_moves).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{assert_eq_set, set, empty_set};
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};
    use crate::task_4::piece::pawn::Pawn;

    #[test]
    fn two_opening_moves_for_e7_pawn() {
        let pawn = Pawn::new(Color::Black, "e7".as_u8().unwrap());
        let legal_moves = set!["e6", "e5"];
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!()))
    }

    #[test]
    fn one_move_for_e6_black_pawn() {
        let pawn = Pawn::new(Color::Black, "e6".as_u8().unwrap());
        let legal_moves = set!["e5"];
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!()))
    }

    #[test]
    fn pawn_moves_should_contain_diagonal_capture_moves() {
        let pawn = Pawn::new(Color::Black, "a5".as_u8().unwrap());
        let opponent_piece_positions = set!["b4"];
        let legal_moves = set!["a4", "b4"];
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &opponent_piece_positions));
    }

    #[test]
    fn pawn_should_not_be_able_to_capture_his_own_pieces() {
        let pawn = Pawn::new(Color::Black, "b5".as_u8().unwrap());
        let your_piece_positions = set!["b5", "c4"];
        let opponent_piece_positions = set!["a4"];
        let legal_moves = set!["a4", "b4"];
        assert_eq_set!(legal_moves, pawn.get_moves(&your_piece_positions, &opponent_piece_positions));
    }
}