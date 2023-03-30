use std::collections::HashSet;
use crate::board_trait::Board;
use crate::enums::Color;
use crate::utils::{get_south_east_diagonal, get_north_east_diagonal};

pub trait Piece {
    fn new(color: Color, position: (u8, u8)) -> Self where Self: Sized;
    fn print(&self) -> char;
    fn get_name(&self) -> String;
    fn get_color(&self) -> Color;
    fn get_position(&self) -> (u8, u8);
    fn move_piece(&mut self, square: (u8, u8));
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)>;
}

pub struct Rook {
    color: Color,
    position: (u8, u8),
    moved: bool
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
    fn new(color: Color, position: (u8, u8)) -> Self where Self: Sized {
        Rook { color, position, moved: false }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => 'R',
            Color::Black => 'r'
        }
    }
    fn get_name(&self) -> String {
        String::from("tårn")
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, square: (u8, u8)) {
        self.position = square;
        self.moved = true;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let move_directions = Rook::get_rook_moves(&self.position);
        board.filter_move_directions(&move_directions, self.color)
    }
}

pub struct Bishop {
    color: Color,
    position: (u8, u8)
}

impl Bishop {
    fn get_bishop_moves(position: &(u8, u8)) -> HashSet<Vec<(u8, u8)>> {
        let (y, x) = *position;

        let south_east: Vec<(u8, u8)> = get_south_east_diagonal(position).into_iter().filter(|&(new_y, new_x)| new_y < y && new_x > x).collect();
        let north_west: Vec<(u8, u8)> = get_south_east_diagonal(position).into_iter().filter(|&(new_y, new_x)| new_y > y && new_x < x).rev().collect();
        let north_east: Vec<(u8, u8)> = get_north_east_diagonal(position).into_iter().filter(|&(new_y, new_x)| new_y > y && new_x > x).collect();
        let south_west: Vec<(u8, u8)> = get_north_east_diagonal(position).into_iter().filter(|&(new_y, new_x)| new_y < y && new_x < x).rev().collect();

        HashSet::from_iter([south_east, north_west, north_east, south_west])
    }
}

impl Piece for Bishop {
    fn new(color: Color, position: (u8, u8)) -> Self where Self: Sized {
        Bishop { color, position }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => 'B',
            Color::Black => 'b'
        }
    }
    fn get_name(&self) -> String {
        String::from("laupar")
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, square: (u8, u8)) {
        self.position = square;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let move_directions = Bishop::get_bishop_moves(&self.position);
        board.filter_move_directions(&move_directions, self.color)
    }
}

pub struct Queen {
    color: Color,
    position: (u8, u8)
}

impl Piece for Queen {
    fn new(color: Color, position: (u8, u8)) -> Self where Self: Sized {
        Queen { color, position }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => 'Q',
            Color::Black => 'q'
        }
    }
    fn get_name(&self) -> String {
        String::from("dronning")
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, square: (u8, u8)) {
        self.position = square;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let mut move_directions = Rook::get_rook_moves(&self.position);
        move_directions.extend(Bishop::get_bishop_moves(&self.position));
        board.filter_move_directions(&move_directions, self.color)
    }
}

pub struct King {
    color: Color,
    position: (u8, u8),
    moved: bool
}

impl King {
    fn can_castle(&self, _board: &Board) -> bool {
        // TODO: Implementer sjekk om rokade er mulig
        false
    }
    fn get_castle_moves(&self) -> HashSet<(u8, u8)> {
        // TODO: Implementer rokadetrekk
        HashSet::new()
    }
    fn get_king_moves(&self) -> HashSet<(u8, u8)> {
        let y = self.position.0 as i8;
        let x = self.position.1 as i8;
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([(y + 1, x - 1),(y + 1, x), (y + 1, x + 1), (y, x - 1), (y, x + 1), (y - 1, x - 1), (y - 1, x), (y - 1, x + 1)]);
        moves.into_iter()
            .filter(|(y, x)| (0..8).contains(y) && (0..8).contains(x))
            .map(|(y, x)| (y as u8, x as u8))
            .collect()
    }
    fn filter_checked_squares(&self, moves: HashSet<(u8, u8)>, _board: &Board) -> HashSet<(u8, u8)> {
        // TODO: Implementer sjekk at ingen andre brikker kan nå feltet i neste trekk
        moves
    }
}

impl Piece for King {
    fn new(color: Color, position: (u8, u8)) -> Self where Self: Sized {
        King { color, position, moved: false }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => 'K',
            Color::Black => 'k'
        }
    }
    fn get_name(&self) -> String {
        String::from("konge")
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, square: (u8, u8)) {
        self.position = square;
        self.moved = true;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let mut moves = self.get_king_moves();
        if self.can_castle(board) {
            moves.extend(self.get_castle_moves());
        }
        moves = moves.into_iter()
            .filter(|square| board.get_square_color(square) != Some(self.color))
            .collect();
        self.filter_checked_squares(moves, board)
    }
}

pub struct Pawn {
    color: Color,
    position: (u8, u8),
    moved: bool
}

impl Pawn {
    fn get_pawn_capture_moves(&self, board: &Board) -> Option<HashSet<(u8, u8)>> {
        // TODO: Add possible en passant captures
        let (y, x) = self.position;
        let mut captures = HashSet::new();
        match self.color {
            Color::White if y < 7 => {
                let capture_y = y + 1;
                if let Some(Color::Black) = board.get_square_color(&(capture_y, x - 1)) {
                    captures.insert((capture_y, x - 1));
                }
                if let Some(Color::Black) = board.get_square_color(&(capture_y, x + 1)) {
                    captures.insert((capture_y, x + 1));
                }
                Some(captures)
            },
            Color::Black if y > 0 => {
                let capture_y = y - 1;
                if let Some(Color::White) = board.get_square_color(&(capture_y, x - 1)) {
                    captures.insert((capture_y, x - 1));
                }
                if let Some(Color::White) = board.get_square_color(&(capture_y, x + 1)) {
                    captures.insert((capture_y, x + 1));
                }
                Some(captures)
            },
            _ => None,
        }
    }
    fn get_pawn_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position;
        match self.color {
            Color::White => {
                match self.moved {
                    true => HashSet::from_iter([(y + 1, x)]),
                    false => HashSet::from_iter([(2, x), (3, x)])
                }
            },
            Color::Black => {
                match self.moved {
                    true => HashSet::from_iter([(y - 1, x)]),
                    false => HashSet::from_iter([(5, x), (4, x)])
                }
            }
        }
    }
}

impl Piece for Pawn {
    fn new(color: Color, position: (u8, u8)) -> Self where Self: Sized {
        Pawn { color, position, moved: false }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => 'P',
            Color::Black => 'p'
        }
    }
    fn get_name(&self) -> String {
        String::from("bonde")
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, square: (u8, u8)) {
        self.position = square;
        self.moved = true;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let mut moves = self.get_pawn_moves();
        if let Some(captures) = self.get_pawn_capture_moves(board) {
            moves.extend(captures)
        }
        moves.into_iter()
            .filter(|square| board.get_square_color(square) != Some(self.color))
            .collect()
    }
}

pub struct Knight {
    color: Color,
    position: (u8, u8),
}

impl Knight {
    fn get_knight_moves(&self) -> HashSet<(u8, u8)> {
        let y = self.position.0 as i8;
        let x = self.position.1 as i8;
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([(y + 2, x - 1), (y - 2, x - 1), (y + 2, x + 1), (y - 2, x + 1), (y - 1, x + 2), (y - 1, x - 2), (y + 1, x + 2), (y + 1, x - 2)]);
        moves.into_iter()
            .filter(|(y, x)| (0..8).contains(y) && (0..8).contains(x))
            .map(|(y, x)| (y as u8, x as u8))
            .collect()
    }
}

impl Piece for Knight {
    fn new(color: Color, position: (u8, u8)) -> Self where Self: Sized {
        Knight { color, position }
    }

    fn print(&self) -> char {
        match self.color {
            Color::White => 'N',
            Color::Black => 'n'
        }
    }
    fn get_name(&self) -> String {
        String::from("springar")
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, square: (u8, u8)) {
        self.position = square;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let moves = self.get_knight_moves();
        moves.into_iter().filter(|square| board.get_square_color(square) != Some(self.color)).collect()
    }
}
