use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;

/// # Oppgave 4
///
/// I denne oppgaven skal vi implementere trekkene til kongen. Du finner metodene som skal
/// implementeres `impl Piece for King {}`-blokken. (Se etter en `todo!()`.
///
/// Kongen kan bevegese til alle nærmeste nabofelter, i alle retninger og diagonalt.
///
/// ## Eksempel
/// Hvit konge i startposisjon `e1` skal kunne gå til `d1`, `d2`, `e2`, `f2`, `f1` (gitt at ingen
/// brikker av samme farge står på disse feltene.
/// ```
/// let king = King::new(Color::White, "e1".as_u8().unwrap());
/// let legal_moves = set!["d1", "d2", "e2", "f2", "f1"];
/// assert_eq_set!(legal_moves, king.get_moves(&empty_set!(), &empty_set!());
/// ```
///
/// PS! En overordnet regel er også at kongen aldri kan gå til et felt hvor han ender opp i sjakk.
/// Dette gjelder også andre brikker, som ikke får stille seg slik at kongen av samme farge havner i
/// sjakk. Dette trenger du ikke ta hensyn til nå – ettersom det er en overordnet regel skal vi løse
/// det senere på et overordnet nivå.
///
/// PPS! I et fullverdig sjakkspill kan kongen kan også *rokere* sammen med tårnet (les mer om
/// [rokade](https://no.wikipedia.org/wiki/Rokade)), men dette sparer vi heller til en ekstraoppgave.
///
/// Se [hint.md](../hint.md) for hint.
#[derive(Clone)]
pub struct King {
    pub color: Color,
    pub position: (u8, u8),
}

pub const KING_NAME: &str = "konge";

impl Piece for King {
    fn new(color: Color, position: (u8, u8)) -> Self {
        King {
            color,
            position,
        }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => '♚',
            Color::Black => '♔',
        }
    }

    fn get_name(&self) -> String {
        String::from(KING_NAME)
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
    use crate::task_4::piece::king::King;

    #[test]
    fn test_king_moves_edge() {
        let king = King::new(Color::White, "e1".as_u8().unwrap());
        let legal_moves = set!["d1", "d2", "e2", "f2", "f1"];
        assert_eq_set!(king.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }

    #[test]
    fn test_king_moves_center() {
        let king = King::new(Color::White, "e5".as_u8().unwrap());
        let legal_moves = set!["d4", "d5", "d6", "e6", "f6", "f5", "f4", "e4"];
        assert_eq_set!(king.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }
}