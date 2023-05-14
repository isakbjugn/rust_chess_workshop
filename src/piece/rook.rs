use std::collections::HashSet;
use crate::color::Color;
use crate::piece::Piece;
use crate::square::MoveDirection;

const ROOK_NAME: &str = "tårn";

#[derive(Clone)]
pub struct Rook {
    pub color: Color,
    pub position: (u8, u8),
}

impl Rook {
    pub(crate) fn get_rook_moves(position: &(u8, u8)) -> HashSet<Vec<(u8, u8)>> {
        let (y, x) = *position;
        let vertical: Vec<(u8, u8)> = vec![(0, x), (1, x), (2, x), (3, x), (4, x), (5, x), (6, x), (7, x)];
        let horizontal: Vec<(u8, u8)> = vec![(y, 0), (y, 1), (y, 2), (y, 3), (y, 4), (y, 5), (y, 6), (y, 7)];

        let north: Vec<(u8, u8)> = vertical.iter().cloned().filter(|&(new_y, _)| new_y > y).collect();
        let south: Vec<(u8, u8)> = vertical.iter().cloned().filter(|&(new_y, _)| new_y < y).rev().collect();
        let east: Vec<(u8, u8)> = horizontal.iter().cloned().filter(|&(_, new_x)| new_x > x).collect();
        let west: Vec<(u8, u8)> = horizontal.iter().cloned().filter(|&(_, new_x)| new_x < x).rev().collect();

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