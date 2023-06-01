use std::collections::HashSet;
use crate::square::{Square, Squares};
use crate::task_2::color::Color;
use crate::task_2::piece::Piece;

mod color;
mod piece;
mod game;
mod board;

/// # Oppgave 2
///
/// Denne oppgaven er en fortsettelse på forrige oppgave, nå skal vi implementere angrepstrekkene til
/// bonden. Her er regelen en gang til:
///  - Bonden kan bevege seg ett felt diagonalt dersom det står en motstander brikke der.
///    Motstander brikken blir da slått.
///
/// Du må utvide `get_moves()` metoden til å støtte dette.
///
/// ## Eksempel
/// Hvit bonde på b4 skal kunne gå til a5, b5 eller c5 dersom det står motstanderbrikker på a5 og c5.
/// ```
/// let pawn = Pawn::new(Color::White, "b4".as_u8().unwrap());
/// let opponent_piece_positions = ["a5", "c5"].as_board_positions();
/// let legal_moves = ["a5", "b5", "c5"].as_board_positions();
/// assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &opponent_piece_positions));
/// ```
pub fn main() {
    println!("Køyrer game::main() i task_2");
    game::main()
}

#[derive(Clone)]
struct Pawn {
    color: Color,
    position: (u8, u8),
}

impl Pawn {
    pub fn get_pawn_moves(&self) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        let moves: HashSet::<(i8, i8)> = match self.color {
            Color::White if y == 1 => HashSet::from_iter([(x, 2), (x, 3)]),
            Color::White => HashSet::from_iter([(x, y + 1)]),
            Color::Black if y == 6 => HashSet::from_iter([(x, 5), (x, 4)]),
            Color::Black => HashSet::from_iter([(x, y - 1)]),
        };
        moves.as_board_positions()
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
        // Du kan gjerne bruke din egen implementasjon fra forrige oppgave her
        let all_pieces = team.union(rival_team).cloned().collect();
        self.get_pawn_moves().difference(&all_pieces).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::assert_eq_set;
    use crate::task_2::color::Color;
    use crate::task_2::Pawn;
    use crate::task_2::piece::Piece;
    use crate::square::{Square, Squares};

    #[test]
    fn pawn_moves_should_contain_diagonal_capture_moves() {
        let pawn = Pawn::new(Color::White, "a4".as_u8().unwrap());
        let opponent_piece_positions = ["b5"].as_board_positions();
        let legal_moves = ["a5", "b5"].as_board_positions();
        assert_eq_set!(legal_moves, pawn.get_moves(&HashSet::from([pawn.position]), &opponent_piece_positions));
    }

    #[test]
    fn pawn_should_not_be_able_to_capture_his_own_pieces() {
        let pawn = Pawn::new(Color::White, "b4".as_u8().unwrap());
        let your_piece_positions = ["b4", "c5"].as_board_positions();
        let opponent_piece_positions = ["a5"].as_board_positions();
        let legal_moves = ["a5", "b5"].as_board_positions();
        assert_eq_set!(legal_moves, pawn.get_moves(&your_piece_positions, &opponent_piece_positions));
    }
}
