use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{MoveDirection, Square};

/// # Oppgave 6
///
/// I denne oppgaven skal vi implementere trekkene til løperen. Du finner metodene som skal
/// implementeres `impl Piece for Bishop {}`-blokken. (Se etter en `todo!()`.
///
/// Løperen kan bevege seg så langs alle diagonaler: Nordøst, nordvest, sørøst, sørvest, helt
/// til den når enden av brettet eller en annen brikke. Løperen kan bevege seg
///
///  a. *til og med* et felt som er tatt av en annen brikke
///  b. til *men ikke med* et felt som er tatt av en brikke med samme farge
///
/// ## Eksempel
/// Hvit løper i startposisjon på `c1`, med en hvit bonde på `b2` og ingen brikker som blokkerer i
/// nordløstlig retning. Løperen skal kunne gå til `d2`, `e3`, `f4`, `g5`, `h6`:
/// ```
/// let bishop = Bishop::new(Color::White, "c1".as_u8().unwrap());
/// let white_pieces = set!["b2"];
/// let black_pieces = empty_set!();
/// let legal_moves = set!["d2", "e3", "f4", "g5", "h6"];
/// assert_eq_set!(legal_moves, bishop.get_moves(&white_pieces, &black_pieces);
/// ```
///
/// Se [hint.md](../hint.md) for hint.
#[derive(Clone)]
pub struct Bishop {
    color: Color,
    position: (u8, u8),
}

const BISHOP_NAME: &str = "laupar";

impl Bishop {

    /// Denne metoden har vi satt ferdig opp for deg, og returnerer den sørøstlige diagonalen som
    /// går gjennom et bestemt felt `position` av typen `&(u8, u8)`, altså fra øvre venstre hjørne
    /// til nedre høyre hjørne.
    ///
    /// Bruk den for å finne gyldige trekk i sørøstlig og nordvestlig retning (som er motsatt av
    /// sørøst). Sjekk ut [hint.md](../hint.md) for innspill!
    fn get_south_east_diagonal(&self) -> Vec<(u8, u8)> {
        let sum = self.position.0 + self.position.1;
        match sum {
            0 => vec![(0, 0)],
            1 => vec![(0, 1), (1, 0)],
            2 => vec![(0, 2), (1, 1), (2, 0)],
            3 => vec![(0, 3), (1, 2), (2, 1), (3, 0)],
            4 => vec![(0, 4), (1, 3), (2, 2), (3, 1), (4, 0)],
            5 => vec![(0, 5), (1, 4), (2, 3), (3, 2), (4, 1), (5, 0)],
            6 => vec![(0, 6), (1, 5), (2, 4), (3, 3), (4, 2), (5, 1), (6, 0)],
            7 => vec![(0, 7), (1, 6), (2, 5), (3, 4), (4, 3), (5, 2), (6, 1), (7, 0)],
            8 => vec![(1, 7), (2, 6), (3, 5), (4, 4), (5, 3), (6, 2), (7, 1)],
            9 => vec![(2, 7), (3, 6), (4, 5), (5, 4), (6, 3), (7, 2)],
            10 => vec![(3, 7), (4, 6), (5, 5), (6, 4), (7, 3)],
            11 => vec![(4, 7), (5, 6), (6, 5), (7, 4)],
            12 => vec![(5, 7), (6, 6), (7, 5)],
            13 => vec![(6, 7), (7, 6)],
            14 => vec![(7, 7)],
            _ => panic!()
        }
    }

    /// Denne metoden har vi satt ferdig opp for deg, og returnerer den nordøstlige diagonalen som
    /// går gjennom et bestemt felt `position` av typen `&(u8, u8)`, altså fra nedre venstre hjørne
    /// til øvre høyre hjørne.
    ///
    /// Bruk den for å finne gyldige trekk i nordøstlig og sørøstlig retning (som er motsatt av
    /// nordøst). Sjekk ut [hint.md](../hint.md) for innspill!
    fn get_north_east_diagonal(&self) -> Vec<(u8, u8)> {
        let difference = self.position.1 as i8 - self.position.0 as i8;
        match difference {
            7 => vec![(0, 7)],
            6 => vec![(0, 6), (1, 7)],
            5 => vec![(0, 5), (1, 6), (2, 7)],
            4 => vec![(0, 4), (1, 5), (2, 6), (3, 7)],
            3 => vec![(0, 3), (1, 4), (2, 5), (3, 6), (4, 7)],
            2 => vec![(0, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 7)],
            1 => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7)],
            0 => vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)],
            -1 => vec![(1, 0), (2, 1), (3, 2), (4, 3), (5, 4), (6, 5), (7, 6)],
            -2 => vec![(2, 0), (3, 1), (4, 2), (5, 3), (6, 4), (7, 5)],
            -3 => vec![(3, 0), (4, 1), (5, 2), (6, 3), (7, 4)],
            -4 => vec![(4, 0), (5, 1), (6, 2), (7, 3)],
            -5 => vec![(5, 0), (6, 1), (7, 2)],
            -6 => vec![(6, 0), (7, 1)],
            -7 => vec![(7, 0)],
            _ => panic!()
        }
    }
}

impl Piece for Bishop {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Bishop {
            color,
            position,
        }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => '♝',
            Color::Black => '♗',
        }
    }

    fn get_name(&self) -> String {
        String::from(BISHOP_NAME)
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

    /// Returnerer et HashSet som inneholder gyldige posisjoner kongen kan flytte til. En posisjon
    /// defineres av et to-tuppel med koordinater, der f.eks (0, 1) korresponderer til feltet A2.
    /// `square.rs` inneholder hjelpefunksjoner for å konvertere f.eks `"a2"` til `(0, 1)` og omvendt.
    ///
    /// # Argumenter
    /// - `team` Referanse til et HashSet som inneholder dine brikkers posisjoner.
    /// - `rival_team` Referanse til et HashSet som inneholder posisjonene til motstanderens brikker.
    ///
    /// Her foreslår vi at du benytter deg av de ferdigimplementerte `get_south_east_diagonal` og
    /// `get_north_east_diagonal` for å beregne feltene løperen kan gå til. Finnes det en måte du
    /// filtrere vektorene som kommer fra disse funksjonene for å representere de fire retningene
    /// løperen kan gå i, og deretter bruke `filter_blocked_squares` som for tårnet?
    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{assert_eq_set, empty_set, set};
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};
    use crate::task_6::piece::bishop::Bishop;

    #[test]
    fn test_bishop_moves_1() {
        let bishop = Bishop::new(Color::White, "a1".as_u8().unwrap());
        let legal_moves = set!["b2", "c3", "d4", "e5", "f6", "g7", "h8"];
        assert_eq_set!(bishop.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }

    #[test]
    fn test_bishop_moves_2() {
        let bishop = Bishop::new(Color::White, "d3".as_u8().unwrap());
        let legal_moves = set![
            "b1", "c2", "e4", "f5", "g6", "h7",
            "a6", "b5", "c4", "e2", "f1"
        ];
        assert_eq_set!(bishop.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }
}