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
    /// I denne oppgaven (4) skal du også finne gyldige forovertrekk for svart bonde.
    fn get_forward_moves(&self, other_pieces: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        match self.color {
            Color::White => {
                let (x, y) = self.position;
                match y {
                    _ if other_pieces.contains(&(x, y + 1)) => HashSet::new(),
                    1 if !other_pieces.contains(&(x, y + 2)) => HashSet::from([(x, 2), (x, 3)]),
                    7 => HashSet::new(),
                    _ => HashSet::from([(x, y + 1)])
                }
            }
            Color::Black => {
                todo!()
            }
        }
    }

    /// Returnerer trekken bonden kan gjøre for å angripe på skrå forover.
    ///
    /// I denne oppgaven (4) skal du også finne gyldige angrepstrekk for svart bonde.
    fn get_capture_moves(&self, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        HashSet::from([(x - 1, y + 1), (x + 1, y + 1)])
            .as_board_positions()
            .intersection(rival_team).cloned().collect()
        // todo!("Returner gyldige angrepstrekk også for svart bonde")
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
    ///
    fn get_moves(
        &self,
        team: &HashSet<(u8, u8)>,
        rival_team: &HashSet<(u8, u8)>,
    ) -> HashSet<(u8, u8)> {
        let other_pieces: HashSet<_> = team.union(rival_team).cloned().collect();
        let forward_moves = self.get_forward_moves(&other_pieces);
        let capture_moves = self.get_capture_moves(rival_team);

        forward_moves.union(&capture_moves).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::finished_game::color::Color;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};
    use crate::task_4::piece::pawn::Pawn;
    use crate::{assert_eq_set, empty_set, set};

    #[test]
    fn two_opening_moves_for_e7_pawn() {
        let pawn = Pawn::new(Color::Black, "e7".as_u8().unwrap());
        let legal_moves = set!["e6", "e5"];
        assert_eq_set!(
            legal_moves,
            pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!())
        )
    }

    #[test]
    fn one_move_for_e6_black_pawn() {
        let pawn = Pawn::new(Color::Black, "e6".as_u8().unwrap());
        let legal_moves = set!["e5"];
        assert_eq_set!(
            legal_moves,
            pawn.get_moves(&HashSet::from([pawn.position]), &empty_set!())
        )
    }

    #[test]
    fn pawn_moves_should_contain_diagonal_capture_moves() {
        let pawn = Pawn::new(Color::Black, "a5".as_u8().unwrap());
        let opponent_piece_positions = set!["b4"];
        let legal_moves = set!["a4", "b4"];
        assert_eq_set!(
            legal_moves,
            pawn.get_moves(&HashSet::from([pawn.position]), &opponent_piece_positions)
        );
    }

    #[test]
    fn pawn_should_not_be_able_to_capture_his_own_pieces() {
        let pawn = Pawn::new(Color::Black, "b5".as_u8().unwrap());
        let your_piece_positions = set!["b5", "c4"];
        let opponent_piece_positions = set!["a4"];
        let legal_moves = set!["a4", "b4"];
        assert_eq_set!(
            legal_moves,
            pawn.get_moves(&your_piece_positions, &opponent_piece_positions)
        );
    }

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

