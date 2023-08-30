use std::collections::HashSet;

use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{Square, Squares};

#[derive(Clone)]
pub struct King {
    pub color: Color,
    pub position: (u8, u8),
}

impl Piece for King {
    fn new(color: Color, position: (u8, u8)) -> Self {
        King { color, position }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => '♚',
            Color::Black => '♔',
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
        #[rustfmt::skip]
        HashSet::from_iter([
            (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
            (x - 1, y    ),             (x + 1, y    ),
            (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        ])
        .as_board_positions()
        .difference(team)
        .cloned()
        .collect()
    }
}
