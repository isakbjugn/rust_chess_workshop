use std::collections::HashSet;
use crate::color::Color;
use crate::piece::bishop::Bishop;
use crate::piece::Piece;
use crate::piece::rook::Rook;
use crate::square::MoveDirection;

const QUEEN_NAME: &str = "dronning";

#[derive(Clone)]
pub struct Queen {
    color: Color,
    position: (u8, u8),
}

impl Piece for Queen {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Queen {
            color,
            position,
        }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♕',
            Color::Black => '♛',
        }
    }

    fn get_name(&self) -> String {
        String::from(QUEEN_NAME)
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
        let mut move_directions = Rook::get_rook_moves(&self.position);
        move_directions.extend(Bishop::get_bishop_moves(&self.position));
        move_directions.iter()
            .flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
    }
}