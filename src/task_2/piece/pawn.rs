#![allow(unused)]
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
    /// Returnerer et HashSet med alle trekkene en bonde kan gjøre forover, avhengig av hvor
    /// brikken står på brettet.
    ///
    /// I denne oppgaven (2) skal du finne forovertrekkene til en hvit bonde (både åpningstrekk og
    /// vanlig bevegelse), og du må ta hensyn til at det kan stå andre brikker i veien.
    ///
    /// For øyeblikket tar ikke denne metoden flere argumenter enn &self, men den burde nok ta inn
    /// de andre brikkene på brettet slik at vi kan sjekke om de er i veien for bondens bevegelse.
    fn get_forward_moves(&self) -> HashSet<(u8, u8)> {
        // todo!("Sjekk om det står brikker i veien for bonden")
        match self.color {
            Color::White => {
                let (x, y) = self.position;
                match y {
                    1 => HashSet::from_iter([(x, 2), (x, 3)]),
                    _ => todo!("Finn vanlige forovertrekk for bonden")
                }
            }
            Color::Black => HashSet::new() // Se bort fra den svarte bonden i denne oppgaven
        }
    }

    /// Returnerer trekken bonden kan gjøre for å angripe på skrå forover. Vi skal se på denne i
    /// oppgave 3.
    fn get_capture_moves(&self) -> HashSet<(u8, u8)> {
        // Denne skal vi se på i oppgave 3
        HashSet::new()
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
    fn get_moves(
        &self,
        team: &HashSet<(u8, u8)>,
        rival_team: &HashSet<(u8, u8)>,
    ) -> HashSet<(u8, u8)> {
        // Her må vi nok gi et argument til self.get_forward_moves() for å kunne sjekke om det står
        // en brikke i veien for bondens trekk
        let forward_moves = self.get_forward_moves();
        let capture_moves = self.get_capture_moves();

        forward_moves.union(&capture_moves).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::finished_game::color::Color;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};
    use crate::task_2::piece::pawn::Pawn;
    use crate::{assert_eq_set, empty_set, set};

    #[test]
    fn two_opening_moves_for_b2_pawn() {
        let pawn = Pawn::new(Color::White, "b2".as_u8().unwrap());
        let legal_moves = set!["b3", "b4"];
        assert_eq_set!(
            legal_moves,
            pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!())
        )
    }

    #[test]
    fn two_opening_moves_for_e2_pawn() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = set!["e3", "e4"];
        assert_eq_set!(
            legal_moves,
            pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!())
        )
    }

    #[test]
    fn one_move_for_e3_white_pawn() {
        let pawn = Pawn::new(Color::White, "e3".as_u8().unwrap());
        let legal_moves = set!["e4"];
        assert_eq_set!(
            legal_moves,
            pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!())
        )
    }

    #[test]
    fn no_moves_for_colliding_pawns() {
        let pawn = Pawn::new(Color::White, "c4".as_u8().unwrap());

        let legal_moves = HashSet::<(u8, u8)>::new();
        assert_eq_set!(
            legal_moves,
            pawn.get_moves(&HashSet::from([pawn.position]), &set!["c5"])
        )
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

