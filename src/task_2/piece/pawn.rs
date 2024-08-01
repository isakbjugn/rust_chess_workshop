#![allow(unused)]
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
        match self.color {
            Color::White => {
                let (x, y) = self.position;
                // Denne linjen var veldig nyttig, men ikke noe jeg husket før jeg sjekke hintene
                // Kanskje det burde vært inkludert i oppgaven fra start?
                let other_pieces: HashSet<_> = team.union(rival_team).collect();
                match y {
                    1 if other_pieces.contains(&(x, y + 1)) => empty_set!(),
                    1 if other_pieces.contains(&(x, y + 2)) => HashSet::from_iter([(x, y + 1)]),
                    1 => HashSet::from_iter([(x, 2), (x, 3)]),
                    _ if other_pieces.contains(&(x, y + 1)) => empty_set!(),
                    _ => HashSet::from_iter([(x, y + 1)]),
                }
            }
            Color::Black => HashSet::new() // Se bort fra den svarte bonden i denne oppgaven
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{assert_eq_set, empty_set, set};
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};
    use crate::task_2::piece::pawn::Pawn;

    #[test]
    fn two_opening_moves_for_b2_pawn() {
        let pawn = Pawn::new(Color::White, "b2".as_u8().unwrap());
        let legal_moves = set!["b3", "b4"];
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!()))
    }

    #[test]
    fn two_opening_moves_for_e2_pawn() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = set!["e3", "e4"];
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!()))
    }

    #[test]
    fn one_move_for_e3_white_pawn() {
        let pawn = Pawn::new(Color::White, "e3".as_u8().unwrap());
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

    #[test]
    fn one_opening_move_for_half_blocked_pawn() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        assert_eq_set!(&set!["e3"], pawn.get_moves(&set!["e2"], &set!["e4"]))
    }
}