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