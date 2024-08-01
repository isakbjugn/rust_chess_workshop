#![allow(unused)]
use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::task_1::piece::Piece;

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
    /// Legg merke til at `self` her er en mutabel referanse `&mut`, ettersom vi skal mutere den
    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target
    }

    /// Returnerer et HashSet som inneholder gyldige posisjoner bonden kan flytte til. En posisjon
    /// defineres av et to-tuppel med koordinater, der f.eks (0, 1) korresponderer til feltet A2.
    /// `square.rs` inneholder hjelpefunksjoner for å konvertere f.eks `"a2"` til `(0, 1)` og omvendt.
    /// Hvorfor står det hint om disse hjelpefunksjonene her?
    ///
    /// # Argumenter
    /// Du trenger ikke å tenke på argumentene til denne oppgaven
    ///
    fn get_moves(&self, _: &HashSet<(u8, u8)>, _: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        match self.color {
            Color::White => {
                HashSet::from_iter([(self.position.0, 2), (self.position.0, 3)])
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
    use crate::i18n::ChessTerm;
    use crate::square::{Square, Squares};
    use crate::task_1::piece::pawn::Pawn;
    use crate::task_1::piece::Piece;

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
    fn can_perform_opening_move() {
        let mut pawn = Pawn::new(Color::White, "d2".as_u8().unwrap());
        pawn.move_piece("d3".as_u8().unwrap());
        assert_eq!(pawn.position, "d3".as_u8().unwrap())
    }

    #[test]
    fn can_get_color() {
        let pawn = Pawn::new(Color::White, "d2".as_u8().unwrap());
        assert_eq!(Color::White, pawn.get_color())
    }
}