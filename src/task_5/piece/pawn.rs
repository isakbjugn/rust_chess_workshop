use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{Square, Squares};

#[derive(Clone)]
pub struct Pawn {
    pub color: Color,
    pub position: (u8, u8),
}

impl Pawn {
    fn get_forward_moves(&self, other_pieces: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position;
        match (self.color, y) {
            (Color::White, 1) if other_pieces.contains(&(x, y + 1)) => HashSet::new(),
            (Color::White, 1) => HashSet::from([(x, 2), (x, 3)]),
            (Color::White, 7) => HashSet::new(),
            (Color::White, _) => HashSet::from([(x, y + 1)]),
            (Color::Black, 6) if other_pieces.contains(&(x, y - 1)) => HashSet::new(),
            (Color::Black, 6) => HashSet::from([(x, 5), (x, 4)]),
            (Color::Black, 0) => HashSet::new(),
            (Color::Black, _) => HashSet::from([(x, y - 1)])
        }.difference(other_pieces).cloned().collect()
    }

    fn get_capture_moves(&self, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        match self.color {
            Color::White => HashSet::from([(x - 1, y + 1), (x + 1, y + 1)]),
            Color::Black => HashSet::from([(x - 1, y - 1), (x + 1, y - 1)]),
        }.as_board_positions().intersection(rival_team).cloned().collect()
    }
}

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
        let all_pieces: HashSet<_> = team.union(rival_team).cloned().collect();
        let forward_moves = self.get_forward_moves(&all_pieces);
        let capture_moves = self.get_capture_moves(rival_team);
        forward_moves.union(&capture_moves).cloned().collect()
    }
}