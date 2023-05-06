use std::collections::HashSet;
#[cfg(feature = "gui")]
use std::fs::read;
#[cfg(feature = "gui")]
use egui_extras::RetainedImage;
use crate::board_trait::Board;
use crate::chess_board::ChessBoard;
use crate::enums::Color;
use crate::squares::Squares;
use crate::utils::{get_south_east_diagonal, get_north_east_diagonal};

pub trait Piece {
    fn new(color: Color, position: (u8, u8)) -> Self where Self: Sized;
    fn print(&self) -> char;
    fn get_name(&self) -> String;
    fn get_color(&self) -> Color;
    fn get_position(&self) -> (u8, u8);
    fn move_piece(&mut self, target: (u8, u8));
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)>;
    fn clone_dyn(&self) -> Box<dyn Piece>;
    #[cfg(feature = "gui")]
    fn get_image(&self) -> &Option<RetainedImage>;
}

impl Clone for Box<dyn Piece> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

const PAWN_NAME: &'static str = "bonde";
const ROOK_NAME: &'static str = "tårn";
const KNIGHT_NAME: &'static str = "springar";
const BISHOP_NAME: &'static str = "laupar";
const QUEEN_NAME: &'static str = "dronning";
pub const KING_NAME: &'static str = "konge";

pub struct Pawn {
    color: Color,
    position: (u8, u8),
    #[cfg(feature = "gui")]
    pub image: Option<RetainedImage>,
}

impl Clone for Pawn {
    fn clone(&self) -> Self {
        Pawn::new(self.color, self.position)
    }
}

impl Pawn {
    fn get_pawn_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position;
        match self.color {
            Color::White => {
                match self.is_initial_position() {
                    false => HashSet::from_iter([(y + 1, x)]),
                    true => HashSet::from_iter([(2, x), (3, x)])
                }
            },
            Color::Black => {
                match self.is_initial_position() {
                    false => HashSet::from_iter([(y - 1, x)]),
                    true => HashSet::from_iter([(5, x), (4, x)])
                }
            }
        }
    }

    fn is_initial_position(&self) -> bool {
        match self.color {
            Color::White => self.position.0 == 1,
            Color::Black => self.position.0 == 6,
        }
    }

    fn get_pawn_capture_moves(&self, board: &Board) -> Option<HashSet<(u8, u8)>> {
        // TODO: Add possible en passant captures
        let (y, x) = self.position;
        let mut captures = HashSet::new();
        match self.color {
            Color::White if y < 7 => {
                let capture_y = y + 1;
                if x > 1 && board.get_square_color(&(capture_y, x - 1)) == Some(Color::Black) {
                    captures.insert((capture_y, x - 1));
                }
                if let Some(Color::Black) = board.get_square_color(&(capture_y, x + 1)) {
                    captures.insert((capture_y, x + 1));
                }
                Some(captures)
            },
            Color::Black if y > 0 => {
                let capture_y = y - 1;
                if x > 1 && board.get_square_color(&(capture_y, x - 1)) == Some(Color::Black) {
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
}

impl Piece for Pawn {
    fn new(color: Color, position: (u8, u8)) -> Self {
        #[cfg(feature = "gui")]
            let image_path = match color {
            Color::White => "assets/bishop-white-48.png",
            Color::Black => "assets/bishop-black-48.png",
        };
        #[cfg(feature = "gui")]
            let image = RetainedImage::from_image_bytes(image_path, &read(image_path).unwrap()).unwrap(); // GUI feature

        Pawn {
            color,
            position,
            #[cfg(feature = "gui")]
            image: Some(image)
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
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let mut moves = self.get_pawn_moves();
        moves.retain(|square| board.get_square_color(square).is_none());
        if let Some(captures) = self.get_pawn_capture_moves(board) {
            moves.extend(captures)
        }
        moves.into_iter()
            .filter(|square| board.get_square_color(square) != Some(self.color))
            .collect()
    }

    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    #[cfg(feature = "gui")]
    fn get_image(&self) -> &Option<RetainedImage> {
        &self.image
    }
}

pub struct Rook {
    pub color: Color,
    pub position: (u8, u8),
    #[cfg(feature = "gui")]
    pub image: Option<RetainedImage>,
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

impl Clone for Rook {
    fn clone(&self) -> Self {
        Rook::new(self.color, self.position)
    }
}

impl Piece for Rook {
    fn new(color: Color, position: (u8, u8)) -> Self {
        #[cfg(feature = "gui")]
        let image_path = match color {
            Color::White => "assets/rook-white-48.png",
            Color::Black => "assets/rook-black-48.png",
        };
        #[cfg(feature = "gui")]
        let image = RetainedImage::from_image_bytes(image_path, &read(image_path).unwrap()).unwrap(); // GUI feature

        Rook {
            color,
            position,
            #[cfg(feature = "gui")]
            image: Some(image)
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
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let move_directions = Rook::get_rook_moves(&self.position);
        board.filter_move_directions(&move_directions, self.color)
    }

    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    #[cfg(feature = "gui")]
    fn get_image(&self) -> &Option<RetainedImage> {
        &self.image
    }
}

pub struct Knight {
    color: Color,
    position: (u8, u8),
    #[cfg(feature = "gui")]
    image: Option<RetainedImage>,
}

impl Clone for Knight {
    fn clone(&self) -> Self {
        Knight::new(self.color, self.position)
    }
}

impl Knight {
    fn get_knight_moves(&self) -> HashSet<(u8, u8)> {
        let y = self.position.0 as i8;
        let x = self.position.1 as i8;
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([(y + 2, x - 1), (y - 2, x - 1), (y + 2, x + 1), (y - 2, x + 1), (y - 1, x + 2), (y - 1, x - 2), (y + 1, x + 2), (y + 1, x - 2)]);
        moves.chess_board_filter()
    }
}

impl Piece for Knight {
    fn new(color: Color, position: (u8, u8)) -> Self {
        #[cfg(feature = "gui")]
            let image_path = match color {
            Color::White => "assets/bishop-white-48.png",
            Color::Black => "assets/bishop-black-48.png",
        };
        #[cfg(feature = "gui")]
            let image = RetainedImage::from_image_bytes(image_path, &read(image_path).unwrap()).unwrap(); // GUI feature

        Knight {
            color,
            position,
            #[cfg(feature = "gui")]
            image: Some(image)
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
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let moves = self.get_knight_moves();
        moves.into_iter().filter(|square| board.get_square_color(square) != Some(self.color)).collect()
    }

    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    #[cfg(feature = "gui")]
    fn get_image(&self) -> &Option<RetainedImage> {
        &self.image
    }
}

pub struct Bishop {
    color: Color,
    position: (u8, u8),
    #[cfg(feature = "gui")]
    image: Option<RetainedImage>,
}

impl Clone for Bishop {
    fn clone(&self) -> Self {
        Bishop::new(self.color, self.position)
    }
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
        #[cfg(feature = "gui")]
        let image_path = match color {
            Color::White => "assets/bishop-white-48.png",
            Color::Black => "assets/bishop-black-48.png",
        };
        #[cfg(feature = "gui")]
        let image = RetainedImage::from_image_bytes(image_path, &read(image_path).unwrap()).unwrap(); // GUI feature
        Bishop {
            color,
            position,
            #[cfg(feature = "gui")]
            image: Some(image)
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
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let move_directions = Bishop::get_bishop_moves(&self.position);
        board.filter_move_directions(&move_directions, self.color)
    }

    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    #[cfg(feature = "gui")]
    fn get_image(&self) -> &Option<RetainedImage> {
        &self.image
    }
}

pub struct Queen {
    color: Color,
    position: (u8, u8),
    #[cfg(feature = "gui")]
    image: Option<RetainedImage>,
}

impl Clone for Queen {
    fn clone(&self) -> Self {
        Queen::new(self.color, self.position)
    }
}

impl Piece for Queen {
    fn new(color: Color, position: (u8, u8)) -> Self {
        #[cfg(feature = "gui")]
        let image_path = match color {
            Color::White => "assets/bishop-white-48.png",
            Color::Black => "assets/bishop-black-48.png",
        };
        #[cfg(feature = "gui")]
        let image = RetainedImage::from_image_bytes(image_path, &read(image_path).unwrap()).unwrap(); // GUI feature

        Queen {
            color,
            position,
            #[cfg(feature = "gui")]
            image: Some(image)
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
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let mut move_directions = Rook::get_rook_moves(&self.position);
        move_directions.extend(Bishop::get_bishop_moves(&self.position));
        board.filter_move_directions(&move_directions, self.color)
    }

    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    #[cfg(feature = "gui")]
    fn get_image(&self) -> &Option<RetainedImage> {
        &self.image
    }
}

pub struct King {
    pub color: Color,
    pub position: (u8, u8),
    #[cfg(feature = "gui")]
    pub image: Option<RetainedImage>,
}

impl Clone for King {
    fn clone(&self) -> Self {
        King::new(self.color, self.position)
    }
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
        moves.chess_board_filter()
    }
}

impl Piece for King {
    fn new(color: Color, position: (u8, u8)) -> Self {
        #[cfg(feature = "gui")]
        let image_path = match color {
            Color::White => "assets/bishop-white-48.png",
            Color::Black => "assets/bishop-black-48.png",
        };
        #[cfg(feature = "gui")]
        let image = RetainedImage::from_image_bytes(image_path, &read(image_path).unwrap()).unwrap(); // GUI feature

        King {
            color,
            position,
            #[cfg(feature = "gui")]
            image: Some(image)
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
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target;
    }
    fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        let mut moves = self.get_king_moves();
        if self.can_castle(board) {
            moves.extend(self.get_castle_moves());
        }
        moves.retain(|square| board.get_square_color(square) != Some(self.color));
        moves
    }

    fn clone_dyn(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }

    #[cfg(feature = "gui")]
    fn get_image(&self) -> &Option<RetainedImage> {
        &self.image
    }
}
