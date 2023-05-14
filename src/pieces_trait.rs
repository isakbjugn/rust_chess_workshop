use std::collections::HashSet;
use crate::enums::Color;
use crate::squares::{MoveDirection, Square, Squares};
use crate::utils::{get_south_east_diagonal, get_north_east_diagonal};

pub trait Piece {
    fn new(color: Color, position: (u8, u8)) -> Self where Self: Sized;
    fn print(&self) -> char;
    fn get_name(&self) -> String;
    fn get_color(&self) -> Color;
    fn get_position(&self) -> &(u8, u8);
    fn move_piece(&mut self, target: (u8, u8));
    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)>;
    fn clone_dyn(&self) -> Box<dyn Piece>;
}

impl Clone for Box<dyn Piece> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

const PAWN_NAME: &str = "bonde";
const ROOK_NAME: &str = "tårn";
const KNIGHT_NAME: &str = "springar";
const BISHOP_NAME: &str = "laupar";
const QUEEN_NAME: &str = "dronning";
pub const KING_NAME: &str = "konge";

#[derive(Clone)]
pub struct Pawn {
    color: Color,
    position: (u8, u8),
}

impl Pawn {
    fn get_pawn_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position.as_i8().unwrap();
        let moves: HashSet::<(i8, i8)> = match self.color {
            Color::White if self.position.0 == 1 => HashSet::from_iter([(2, x), (3, x)]),
            Color::White => HashSet::from_iter([(y + 1, x)]),
            Color::Black if self.position.0 == 6 => HashSet::from_iter([(5, x), (4, x)]),
            Color::Black => HashSet::from_iter([(y - 1, x)]),
        };
        moves.as_board_positions()
    }
    fn get_pawn_capture_moves(&self) -> HashSet<(u8, u8)> {
        // TODO: Add possible en passant captures
        let (y, x) = self.position.as_i8().unwrap();
        let capture_moves: HashSet<(i8, i8)> = match self.color {
            Color::White => HashSet::from_iter([(y + 1, x - 1), (y + 1, x + 1)]),
            Color::Black => HashSet::from_iter([(y - 1, x - 1), (y - 1, x + 1)]),
        };
        capture_moves.as_board_positions()
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
            Color::White => '♙',
            Color::Black => '♟',
        }
    }
    fn get_name(&self) -> String {
        String::from(PAWN_NAME)
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
        let all_pieces = team.union(rival_team).cloned().collect();
        let moves: HashSet<(u8, u8)> = self.get_pawn_moves().difference(&all_pieces).cloned().collect();
        let capture_moves: HashSet<(u8, u8)> = self.get_pawn_capture_moves().intersection(rival_team).cloned().collect();
        moves.union(&capture_moves).cloned().collect()
    }
    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct Rook {
    pub color: Color,
    pub position: (u8, u8),
}

impl Rook {
    fn get_rook_moves(position: &(u8, u8)) -> HashSet<Vec<(u8, u8)>> {
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

    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct Knight {
    color: Color,
    position: (u8, u8),
}

impl Knight {
    fn get_knight_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position.as_i8().unwrap();
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([(y + 2, x - 1), (y - 2, x - 1), (y + 2, x + 1), (y - 2, x + 1), (y - 1, x + 2), (y - 1, x - 2), (y + 1, x + 2), (y + 1, x - 2)]);
        moves.as_board_positions()
    }
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
            Color::White => '♘',
            Color::Black => '♞',
        }
    }
    fn get_name(&self) -> String {
        String::from(KNIGHT_NAME)
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
    fn get_moves(&self, team: &HashSet<(u8, u8)>, _: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        self.get_knight_moves().difference(team).cloned().collect()
    }

    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct Bishop {
    color: Color,
    position: (u8, u8),
}

impl Bishop {
    fn get_bishop_moves(position: &(u8, u8)) -> HashSet<Vec<(u8, u8)>> {
        let (y, x) = *position;
        let se_diag = get_south_east_diagonal(position);
        let ne_diag = get_north_east_diagonal(position);

        let south_east: Vec<(u8, u8)> = se_diag.iter().cloned().filter(|&(new_y, new_x)| new_y < y && new_x > x).collect();
        let north_west: Vec<(u8, u8)> = se_diag.iter().cloned().filter(|&(new_y, new_x)| new_y > y && new_x < x).rev().collect();
        let north_east: Vec<(u8, u8)> = ne_diag.iter().cloned().filter(|&(new_y, new_x)| new_y > y && new_x > x).collect();
        let south_west: Vec<(u8, u8)> = ne_diag.iter().cloned().filter(|&(new_y, new_x)| new_y < y && new_x < x).rev().collect();

        HashSet::from_iter([south_east, north_west, north_east, south_west])
    }
}

impl Piece for Bishop {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Bishop {
            color,
            position,
        }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => '♗',
            Color::Black => '♝',
        }
    }
    fn get_name(&self) -> String {
        String::from(BISHOP_NAME)
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
        Bishop::get_bishop_moves(&self.position).iter()
            .flat_map(|v| v.filter_blocked_squares(team, rival_team)).collect()
    }

    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

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

    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct King {
    pub color: Color,
    pub position: (u8, u8),
}

impl King {
    fn get_king_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position.as_i8().unwrap();
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([(y + 1, x - 1), (y + 1, x), (y + 1, x + 1), (y, x - 1), (y, x + 1), (y - 1, x - 1), (y - 1, x), (y - 1, x + 1)]);
        moves.as_board_positions()
    }
}

impl Piece for King {
    fn new(color: Color, position: (u8, u8)) -> Self {
        King {
            color,
            position,
        }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => '♔',
            Color::Black => '♚',
        }
    }

    fn get_name(&self) -> String {
        String::from(KING_NAME)
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

    fn get_moves(&self, team: &HashSet<(u8, u8)>, _: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        self.get_king_moves().difference(team).cloned().collect()
    }

    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::enums::{Color};
    use crate::pieces_trait::{Pawn, Piece};
    use crate::squares::{Square, Squares};

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