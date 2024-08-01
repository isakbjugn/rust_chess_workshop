#![allow(unused)]
use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::MoveDirection;

#[derive(Clone)]
pub struct Rook {
    pub color: Color,
    pub position: (u8, u8),
}

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
        let (x, y) = self.position;
        let vertical = vec![(x, 0), (x, 1), (x, 2), (x, 3), (x, 4), (x, 5), (x, 6), (x, 7)];
        let horizontal = vec![(0, y), (1, y), (2, y), (3, y), (4, y), (5, y), (6, y), (7, y)];
        
        let north: Vec<_> = vertical.iter().cloned().skip_while(|&(_, new_y)| new_y <= y).collect();
        let south: Vec<_> = vertical.iter().cloned().take_while(|&(_, new_y)| new_y < y).collect();
        let east: Vec<_> = horizontal.iter().cloned().skip_while(|&(new_x, _)| new_x <= x).collect();
        let west: Vec<_> = horizontal.iter().cloned().take_while(|&(new_x, _)| new_x < x).collect();
        
        HashSet::<Vec<(u8, u8)>>::from_iter([north, south, east, west])
            .iter().flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{assert_eq_set, empty_set, set};
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};
    use crate::task_7::piece::rook::Rook;

    #[test]
    fn test_rook_moves_1() {
        let rook = Rook::new(Color::White, "a1".as_u8().unwrap());
        let legal_moves = set![
            "a2", "a3", "a4", "a5", "a6", "a7", "a8",
            "b1", "c1", "d1", "e1", "f1", "g1", "h1",
        ];
        assert_eq_set!(rook.get_moves(&empty_set!(), &empty_set!()), legal_moves)
    }

    #[test]
    fn test_rook_moves_2() {
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