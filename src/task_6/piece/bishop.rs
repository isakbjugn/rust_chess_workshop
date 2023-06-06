use std::collections::HashSet;
use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;

/// # Oppgave 6
///
/// I denne oppgaven skal vi implementere trekkene til springeren. Du finner metodene som skal
/// implementeres `impl Piece for King {}`-blokken. (Se etter en `todo!()`.
///
/// Springeren kan bevege seg så langs alle diaogonaler: Nordøst, nordvest, sørøst, sørvest, helt
/// til den når enden av brettet eller en annen brikke. Springeren kan bevege seg
///  a. *til og med* et felt som er tatt av en annen brikke
///  b. til *men ikke med* et felt som er tatt av en brikke med samme farge
///
/// ## Eksempel
/// Hvitt tårn i startposisjon på `c1`, med en hvit bonde på `b2` og ingen brikker som blokkerer i
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
    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{assert_eq_set, empty_set, set};
    use crate::finished_game::color::Color;
    use crate::task_6::piece::bishop::Bishop;
    use crate::task_6::piece::Piece;
    use crate::square::{Square, Squares};

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