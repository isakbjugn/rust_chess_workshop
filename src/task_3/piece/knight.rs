use std::collections::HashSet;
use crate::square::{Square, Squares};
use crate::color::Color;
use crate::task_3::piece::Piece;

/// # Oppgave 3
///
/// I denne oppgaven skal vi implementere trekkene til springeren (hest). Dette gjøres ved å
/// implementere metodene som er definert i `impl Piece for Knight {}`-blokken. Se etter metoden
/// som inneholder en `todo!()`.
///
/// Springeren beveger seg i noe man kan kalle et `L`-mønster:
///  - Hvert trekk forflytter springeren to steg i én retning og ett steg i rett vinkel fra den
/// første retningen
///  - Springeren kan kun gå til felter som er tomme, eller der en brikke av motsatt farge står
///
/// ## Eksempel
/// Hvit springer i startposisjon `b1` skal kunne gå til `a3` og `c3`:
/// ```
/// let knight = Knight::new(Color::White, "b1".as_u8().unwrap());
/// let legal_moves = set!["a3", "c3"];
/// assert_eq_set!(legal_moves, pawn.get_moves(&empty_set!(), &empty_set!());
/// ```
///
/// Se [hint.md](../hint.md) for hint.
#[derive(Clone)]
pub struct Knight {
    position: (u8, u8),
    color: Color,
}

const KNIGHT_NAME: &str = "springar";

impl Piece for Knight {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Knight {
            color,
            position,
        }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => '♘',
            Color::Black => '♞',
        }
    }

    fn get_name(&self) -> String {
        String::from(KNIGHT_NAME)
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

    /// Returnerer et HashSet som inneholder gyldige posisjoner springeren kan flytte til. En posisjon
    /// defineres av et to-tuppel med koordinater, der f.eks (0, 1) korresponderer til feltet A2.
    /// `square.rs` inneholder hjelpefunksjoner for å konvertere f.eks `"a2"` til `(0, 1)` og omvendt.
    ///
    /// # Argumenter
    /// - `team` Referanse til et HashSet som inneholder dine brikkers posisjoner.
    /// - `rival_team` Referanse til et HashSet som inneholder posisjonene til motstanderens brikker.
    ///
    fn get_moves(&self, _team: &HashSet<(u8, u8)>, _rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{assert_eq_set, empty_set, set};
    use crate::square::{Square, Squares};
    use crate::color::Color;
    use crate::task_3::piece::knight::Knight;
    use crate::task_3::piece::Piece;

    #[test]
    fn knight_moves_in_start_position() {
        let knight = Knight::new(Color::White, "b1".as_u8().unwrap());
        let legal_moves = set!["a3", "c3", "d2"];
        assert_eq_set!(legal_moves, knight.get_moves(&HashSet::from([knight.position]), &empty_set!()));
    }

    #[test]
    fn knight_moves_in_center() {
        let knight = Knight::new(Color::White, "d5".as_u8().unwrap());
        let legal_moves = set!["b4", "c3", "e3", "f4", "c7", "b6", "e7", "f6"];
        assert_eq_set!(legal_moves, knight.get_moves(&HashSet::from([knight.position]), &empty_set!()));
    }
}
