use std::collections::HashSet;
use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::MoveDirection;

#[derive(Clone)]
pub struct Rook {
    pub color: Color,
    pub position: (u8, u8),
}

const ROOK_NAME: &str = "tårn";

impl Rook {
    pub(crate) fn get_rook_moves(position: &(u8, u8)) -> HashSet<Vec<(u8, u8)>> {
        let (x, y) = *position;
        let vertical: Vec<(u8, u8)> = vec![(x, 0), (x, 1), (x, 2), (x, 3), (x, 4), (x, 5), (x, 6), (x, 7)];
        let horizontal: Vec<(u8, u8)> = vec![(0, y), (1, y), (2, y), (3, y), (4, y), (5, y), (6, y), (7, y)];

        let north: Vec<(u8, u8)> = vertical.iter().cloned().filter(|&(_, new_y)| new_y > y).collect();
        let south: Vec<(u8, u8)> = vertical.iter().cloned().filter(|&(_, new_y)| new_y < y).rev().collect();
        let east: Vec<(u8, u8)> = horizontal.iter().cloned().filter(|&(new_x, _)| new_x > x).collect();
        let west: Vec<(u8, u8)> = horizontal.iter().cloned().filter(|&(new_x, _)| new_x < x).rev().collect();

        HashSet::from_iter([north, south, east, west])
    }
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
            Color::White => '♖',
            Color::Black => '♜',
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
        Rook::get_rook_moves(&self.position).iter()
            .flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{assert_eq_set, empty_set, set};
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::rook::Rook;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};

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