use std::collections::HashSet;
use crate::task_1::color::Color;
use crate::task_1::piece::Piece;

mod color;
mod piece;
mod game;
mod board;

/// # Oppgave 1
///
/// Denne oppgaven går ut på å implementere bonden sine trekk. Dette gjøres ved å implementere
/// metodene som står definert inni `impl Piece for Pawn {}` blokken. (Se etter `todo!()`)
/// `Piece` er et slags interface, som kalles trait i rust (mer om det senere).
///
/// Bonden kan kun bevege seg rett frem (dersom feltet er ledig), og kan kun bevege seg ett felt om gangen.
/// Det er to unntak:
///  - Hvis det er det første trekket til bonden kan den også velge å bevege seg to felt fremover.
///  - Bonden kan bevege seg ett felt diagonalt dersom det står en motstander brikke der.
///    Motstander brikken blir da slått. (Dette venter vi med å implementere til neste senere oppgave)
///
/// `Pawn`-strukten inneholoder allerede metodene `new` og `print`. I denne oppgaven skal vi utvide
/// `Pawn` med flere metoder vi har behov for.
/// Vi skal lage tre nyttefunksjoner, for å ha tilgang til private felt:
///    - `get_name` (gir ut brikkens navn, finnes som strengkonstant)
///    - `get_color` (gir ut brikkens farge)
///    - `get_position` (gir ut brikkens posisjon)
///
/// Samt to metoder vi trenger for å flytte bonden:
///    - `move_piece` (endrer brikkens posisjon)
///    - `get_moves` (henter ut gyldige felt en brikke kan flytte til)
///
/// Oppgaven er fullført når testene kjører grønt.
/// Det kan være nyttig å først kjøre `cargo run` for å få printet ut et tomt sjakkbrett for å letter
/// kune visualisere posisjoner.
///
/// Se [hint.md](./hint.md) for hint.
///
/// ## Les mer om:
///    - [Metoder som muterer](https://doc.rust-lang.org/book/ch05-03-method-syntax.html?#defining-methods)
///    - [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
///    - [Referanser](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
pub fn main() {
    println!("Køyrer game::main() i task_1");
    game::main()
}

#[derive(Clone)]
struct Pawn {
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
    /// defineres av et to-tuppel med koordinater, der f.eks (0, 2) korresponderer til feltet A2.
    /// `square.rs` inneholder hjelpefunksjoner for å konvertere f.eks `"a2"` til `(0, 2)` og omvendt.
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
    use crate::task_1::color::Color;
    use crate::task_1::Pawn;
    use crate::task_1::piece::Piece;
    use crate::square::{Square, Squares};

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
}
