pub mod pawn;
pub mod rook;
pub mod knight;
pub mod bishop;
pub mod queen;
pub mod king;

use std::collections::HashSet;
use dyn_clonable::clonable;
use crate::finished_game::color::Color;

#[clonable]
pub trait Piece: Clone {
    fn new(color: Color, position: (u8, u8)) -> Self where Self: Sized;
    fn print(&self) -> char;
    fn get_name(&self) -> String;
    fn get_color(&self) -> Color;
    fn get_position(&self) -> &(u8, u8);
    fn move_piece(&mut self, target: (u8, u8));
    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)>;
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::pawn::Pawn;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};

    #[test]
    fn test_white_pawn_top_row() {
        let pawn = Pawn::new(Color::White, (7, 0));
        let legal_moves = HashSet::<(u8, u8)>::new();
        let positions = HashSet::new();
        assert_eq!(pawn.get_moves(&positions, &positions), legal_moves)
    }

    #[test]
    fn test_black_pawn_bottom_row() {
        let pawn = Pawn::new(Color::Black, (0, 0));
        let positions = HashSet::new();
        let legal_moves = HashSet::<(u8, u8)>::new();
        assert_eq!(pawn.get_moves(&positions, &positions), legal_moves)
    }

    #[test]
    fn two_moves_for_e2_opening_move() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = ["e3", "e4"].as_board_positions();
        assert_eq!(pawn.get_pawn_moves(), legal_moves)
    }

    #[test]
    fn two_capture_moves_for_e2_opening_move() {
        let pawn = Pawn::new(Color::White, "e2".as_u8().unwrap());
        let legal_moves = ["d3", "f3"].as_board_positions();
        assert_eq!(pawn.get_pawn_capture_moves(), legal_moves)
    }
}