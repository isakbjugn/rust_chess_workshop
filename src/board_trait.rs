use std::collections::{HashMap, HashSet};
use crate::pieces_trait::{Piece, Pawn, Rook, Knight, Bishop, Queen, King};
use colored::Colorize;
use egui_extras::RetainedImage;
use crate::enums::Color;

pub struct Board {
    pub chess_board_image: RetainedImage,
    pieces: HashMap<(u8, u8), Box<dyn Piece>>
}

impl Board {
    pub fn new() -> Board {
        let mut pieces = Vec::<Box<dyn Piece>>::new();
        let teams: Vec<(Color, u8, u8)> = vec![(Color::White, 0, 1), (Color::Black, 7, 6)];
        for &(color, officer_row, pawn_row) in &teams {
            for col in 0..=7 {
                pieces.push( Box::new(Pawn::new(color,(pawn_row, col))));
            }
            pieces.push(Box::new(Rook::new(color,(officer_row, 0))));
            pieces.push(Box::new(Knight::new(color,(officer_row, 1))));
            pieces.push(Box::new(Bishop::new(color,(officer_row, 2))));
            pieces.push(Box::new(Queen::new(color,(officer_row, 3))));
            pieces.push(Box::new(King::new(color,(officer_row, 4))));
            pieces.push(Box::new(Bishop::new(color,(officer_row, 5))));
            pieces.push(Box::new(Knight::new(color,(officer_row, 6))));
            pieces.push(Box::new(Rook::new(color,(officer_row, 7))));
        }
        Board {
            chess_board_image: RetainedImage::from_image_bytes(
                "chess_board",
                include_bytes!("../assets/board-384-brown.png"),
            ).unwrap(),
            pieces: pieces.into_iter().map(|piece| (piece.get_position(), piece)).collect()
        }
    }

    pub fn get_piece_name(&self, position: &(u8, u8)) -> String {
        self.pieces.get(position).map(|piece| piece.get_name()).unwrap()
    }

    pub fn get_square_color(&self, position: &(u8, u8)) -> Option<Color> {
        self.pieces.get(position).map(|piece| piece.get_color())
    }

    pub fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)> {
        self.pieces.get(position).unwrap().get_moves(&self)
    }

    pub fn filter_move_directions(&self, move_directions: &HashSet<Vec<(u8, u8)>>, color: Color) -> HashSet<(u8, u8)> {
        let mut moves = HashSet::new();
        for line in move_directions {
            'direction: for &square in line {
                match self.get_square_color(&square) {
                    Some(c) if c != color => {
                        moves.insert(square);
                        break 'direction;
                    },
                    Some(_) => break 'direction,
                    None => {
                        moves.insert(square);
                    }
                }
            }
        }
        moves
    }

    pub fn capture(&mut self, position: &(u8, u8), square: (u8, u8)) {
        println!("{} fra {:?} fangar {} på {:?}", self.get_piece_name(&position), position, self.get_piece_name(&square), square);
        self.move_piece(position, square);
    }

    pub fn print(&self) {
        let board = self.create_board();
        println!("   {:_<33}", "");
        for (y, row) in board.iter().rev().enumerate() {
            print!("{}  ", 8 - y);
            for piece in row {
                match *piece {
                    '_' => print!("|   "),
                    c => print!("| {} ", c)
                }
            }
            println!("|")
        }
        println!("   {:\u{035E}<33}", "");
        println!("     A   B   C   D   E   F   G   H");
    }

    pub fn print_with_legal_moves(&self, legal_squares: &HashSet<(u8, u8)>) {
        let board = self.create_board();

        println!("   {:_<33}", "");
        for (y, row) in board.iter().rev().enumerate() {
            print!("{}  ", 8 - y);
            for (x, piece) in row.iter().enumerate() {
                match *piece {
                    '_' if legal_squares.contains(&(7 - y as u8, x as u8)) => print!("| {} ", "□".green()),
                    '_' => print!("|   "),
                    c if legal_squares.contains(&(7 - y as u8, x as u8)) => print!("| {} ", c.to_string().red()),
                    c => print!("| {} ", c)
                }
            }
            println!("|")
        }
        println!("   {:͞<33}", ""); // \u{035E}
        println!("     A   B   C   D   E   F   G   H");
    }

    fn create_board(&self) -> Vec<Vec<char>> {
        let mut board = vec![vec!['_'; 8]; 8];
        for (position, piece) in &self.pieces {
            board[position.0 as usize][position.1 as usize] = piece.print();
        }
        board
    }

    pub fn move_piece(&mut self, position: &(u8, u8), square: (u8, u8)) {
        let mut moving_piece = self.pieces.remove(position).unwrap();
        moving_piece.move_piece(square);
        self.pieces.remove(&square);
        self.pieces.insert(square, moving_piece);
    }
}