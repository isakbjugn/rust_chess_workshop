use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;

/// # Oppgave 5
///
/// I denne oppgaven skal vi implementere trekkene til tårnet. Du finner metodene som skal
/// implementeres `impl Piece for King {}`-blokken. (Se etter en `todo!()`.
///
/// Tårnet kan bevege seg så langt den vil mot nord, sør, øst og vest på brettet, helt til den når
/// enden av brettet eller en annen brikke. Tårnet kan bevege seg frem
///  a. *til og med* et felt som er tatt av en annen brikke
///  b. til *men ikke med* et felt som er tatt av en brikke med samme farge
///
/// ## Eksempel
/// Hvitt tårn i startposisjon på `a1`, med en svart bonde på `a5` og hvit løper på `c1` skal kunne
/// gå  skal kunne gå til `a2`, `a3`, `a4`, `a5`, `b1`:
/// ```
/// let rook = Rook::new(Color::White, "a1".as_u8().unwrap());
/// let white_pieces = set!["c1"];
/// let black_pieces = set!["a5"];
/// let legal_moves = set!["a2", "a3", "a4", "a5", "b1"];
/// assert_eq_set!(legal_moves, rook.get_moves(&white_pieces, &black_pieces);
/// ```
///
/// PS! I liket med kongen i oppgave 4 venter vi med å implementere rokade, som vanligvis også
/// omfatter tårnet (les mer om [rokade](https://no.wikipedia.org/wiki/Rokade)).
///
/// Se [hint.md](../hint.md) for hint.
#[derive(Clone)]
pub struct Rook {
    pub color: Color,
    pub position: (u8, u8),
}

const ROOK_NAME: &str = "tårn";

impl Piece for Rook {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Rook {
            color,
            position,
        }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♜',
            Color::Black => '♖',
        }
    }
    fn get_name(&self) -> String {
        String::from(ROOK_NAME)
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
    use crate::task_5::piece::rook::Rook;

    #[test]
    fn test_queen_moves_1() {
        let rook = Rook::new(Color::White, "a1".as_u8().unwrap());
        let legal_moves = set![
            "a2", "a3", "a4", "a5", "a6", "a7", "a8",
            "b1", "c1", "d1", "e1", "f1", "g1", "h1",
        ];
        assert_eq_set!(rook.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }

    #[test]
    fn test_queen_moves_2() {
        let rook = Rook::new(Color::White, "d4".as_u8().unwrap());
        let legal_moves = set![
            "c4", "b4", "a4",
            "d5", "d6", "d7", "d8",
            "e4", "f4", "g4", "h4",
            "d3", "d2",  "d1",
        ];
        assert_eq_set!(rook.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }
}