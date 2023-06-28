use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::task_1::piece::Piece;

#[derive(Clone)]
pub struct Pawn {
    color: Color,
    position: (u8, u8),
}

const PAWN_NAME: &str = "bonde";

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

    /// Returnerer en tekstlig representasjon av bonden (Se PAWN_NAME)
    /// Denne brukes ikke i task_1 men kreves for å oppfylle kontrakten til Piece
    fn get_name(&self) -> String {
        todo!()
    }

    fn get_color(&self) -> Color {
        todo!()
    }

    fn get_position(&self) -> &(u8, u8) {
        &self.position
    }

    /// Flytter brikken til posisjonen indikert av `target` ved å mutere sin egen posisjon
    /// Legg merke til at `self` her er en mutabel referanse `&mut`, ettersom vi skal mutere den
    fn move_piece(&mut self, target: (u8, u8)) {
        todo!()
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
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{assert_eq_set, empty_set, set};
    use crate::finished_game::color::Color;
    use crate::square::{Square, Squares};
    use crate::task_1::piece::pawn::Pawn;
    use crate::task_1::piece::Piece;

    #[test]
    fn two_opening_moves_for_e2_pawn() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = set!["e3", "e4"];
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!()))
    }

    #[test]
    fn two_opening_moves_for_e7_pawn() {
        let pawn = Pawn::new(Color::Black, "e7".as_u8().unwrap());
        let legal_moves = set!["e5", "e6"];
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!()))
    }

    #[test]
    fn one_moves_for_b4_black_pawn() {
        let pawn = Pawn::new(Color::Black, "a4".as_u8().unwrap());
        let legal_moves = set!["a3"];
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!()))
    }

    #[test]
    fn one_move_for_e3_white_pawn() {
        let mut pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        pawn.move_piece("e3".as_u8().unwrap());

        let legal_moves = set!["e4"];
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!()))
    }

    #[test]
    fn no_moves_for_colliding_pawns() {
        let pawn = Pawn::new(Color::White, "c4".as_u8().unwrap());

        let legal_moves = HashSet::<(u8, u8)>::new();
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &set!["c5"]))
    }

    #[test]
    fn no_opening_moves_for_blocked_pawn() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        assert_eq_set!(empty_set!(), pawn.get_moves(&set!["e2"], &set!["e3"]))
    }
}