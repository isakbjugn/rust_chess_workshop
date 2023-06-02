use std::collections::HashSet;
use crate::color::Color;
use crate::square::{Square, Squares};
use crate::task_3::piece::Piece;

#[derive(Clone)]
pub struct Pawn {
    pub color: Color,
    pub position: (u8, u8),
}

impl Pawn {
    pub fn get_pawn_moves(&self, other_pieces: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position;
        match (self.color, y) {
            (Color::White, 1) if other_pieces.contains(&(x, y + 1)) => HashSet::new(),
            (Color::White, 1) => HashSet::from_iter([(x, 2), (x, 3)]),
            (Color::White, _) => HashSet::from_iter([(x, y + 1)]),
            (Color::Black, 6) if other_pieces.contains(&(x, y - 1)) => HashSet::new(),
            (Color::Black, 6) => HashSet::from_iter([(x, 5), (x, 4)]),
            (Color::Black, _) => HashSet::from_iter([(x, y - 1)])
        }.difference(other_pieces).cloned().collect()
    }

    pub fn get_pawn_capture_moves(&self, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        match self.color {
            Color::White => HashSet::from_iter([(x - 1, y + 1), (x + 1, y + 1)]),
            Color::Black => HashSet::from_iter([(x - 1, y - 1), (x + 1, y - 1)]),
        }.as_board_positions().intersection(rival_team).cloned().collect()
    }
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

    /// Returnerer en tekstlig representasjon av bonden
    fn get_name(&self) -> String {
        String::from(PAWN_NAME)
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
    /// defineres av et to-tuppel med koordinater, der f.eks (0, 2) korresponderer til feltet A2.
    /// `square.rs` inneholder hjelpefunksjoner for å konvertere f.eks `"a2"` til `(0, 2)` og omvendt.
    ///
    /// # Argumenter
    /// - `team` Referanse til et HashSet som inneholder dine brikkers posisjoner.
    /// - `rival_team` Referanse til et HashSet som inneholder posisjonene til motstanderens brikker.
    ///
    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        // Du kan gjerne bruke din egen implementasjon fra oppgave 1 og 2 her
        let all_pieces: HashSet<_> = team.union(rival_team).cloned().collect();
        let moves = self.get_pawn_moves(&all_pieces);
        let capture_moves = self.get_pawn_capture_moves(rival_team);
        moves.union(&capture_moves).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{assert_eq_set, set};
    use crate::color::Color;
    use crate::square::{Square, Squares};
    use crate::task_3::piece::pawn::Pawn;
    use crate::task_3::piece::Piece;

    #[test]
    fn pawn_moves_should_contain_diagonal_capture_moves() {
        let pawn = Pawn::new(Color::White, "a4".as_u8().unwrap());
        let opponent_piece_positions = set!["b5"];
        let legal_moves = set!["a5", "b5"];
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &opponent_piece_positions));
    }

    #[test]
    fn pawn_should_not_be_able_to_capture_his_own_pieces() {
        let pawn = Pawn::new(Color::White, "b4".as_u8().unwrap());
        let your_piece_positions = set!["b4", "c5"];
        let opponent_piece_positions = set!["a5"];
        let legal_moves = set!["a5", "b5"];
        assert_eq_set!(legal_moves, pawn.get_moves(&your_piece_positions, &opponent_piece_positions));
    }
}
