use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{Square, Squares};

#[derive(Clone)]
pub struct Knight {
    position: (u8, u8),
    color: Color,
}

impl Piece for Knight {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Knight {
            color,
            position,
        }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => '♞',
            Color::Black => '♘',
        }
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
    fn get_moves(&self, team: &HashSet<(u8, u8)>, _rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([
                            (x - 1, y + 2), (x + 1, y + 2),
            (x - 2, y + 1),                                 (x + 2, y + 1),

            (x - 2, y - 1),                                 (x + 2, y - 1),
                            (x - 1, y - 2), (x + 1, y - 2),
        ]);
        moves.as_board_positions().difference(team).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{assert_eq_set, empty_set, set};
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};
    use crate::task_5::piece::knight::Knight;

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
