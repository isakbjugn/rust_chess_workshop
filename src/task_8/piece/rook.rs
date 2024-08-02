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
    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position;
        let vertical: Vec<(u8, u8)> = vec![(x, 0), (x, 1), (x, 2), (x, 3), (x, 4), (x, 5), (x, 6), (x, 7)];
        let horizontal: Vec<(u8, u8)> = vec![(0, y), (1, y), (2, y), (3, y), (4, y), (5, y), (6, y), (7, y)];

        let north: Vec<(u8, u8)> = vertical.iter().cloned().filter(|&(_, new_y)| new_y > y).collect();
        let south: Vec<(u8, u8)> = vertical.iter().cloned().filter(|&(_, new_y)| new_y < y).rev().collect();
        let east: Vec<(u8, u8)> = horizontal.iter().cloned().filter(|&(new_x, _)| new_x > x).collect();
        let west: Vec<(u8, u8)> = horizontal.iter().cloned().filter(|&(new_x, _)| new_x < x).rev().collect();

        [north, south, east, west]
            .iter().flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
    }
}