use std::collections::{HashMap, HashSet};
use crate::pieces_trait::{Piece, Pawn, Rook, Knight, Bishop, Queen, King};
use colored::Colorize;
use crate::enums::Color;

pub struct Board {
    pieces: HashMap<(u8, u8), Box<dyn Piece>>
}

impl Board {
    pub fn new() -> Board {
        let mut pieces = Vec::<Box<dyn Piece>>::new();
        let teams: Vec<(Color, u8, u8)> = vec![(Color::White, 0, 1), (Color::Black, 7, 6)];
        for &(color, officer_row, pawn_row) in teams.iter() {
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
        Board { pieces: pieces.into_iter().map(|piece| (piece.get_position(), piece)).collect() }
    }

    pub fn get_piece_name(&self, position: &(u8, u8)) -> String {
        self.pieces.get(position).map(|piece| piece.get_name()).unwrap()
    }

    pub fn get_square_color(&self, position: &(u8, u8)) -> Option<Color> {
        self.pieces.get(position).map(|piece| piece.get_color())
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

    pub fn print_with_legal_moves(&self, piece: &(u8, u8)) {
        let board = self.create_board();
        let legal_moves = self.pieces.get(piece).unwrap().get_moves(&self);

        println!("   {:_<33}", "");
        for (y, row) in board.iter().rev().enumerate() {
            print!("{}  ", 8 - y);
            for (x, piece) in row.iter().enumerate() {
                match *piece {
                    '_' if legal_moves.contains(&(7 - y as u8, x as u8)) => print!("| {} ", "□".green()),
                    '_' => print!("|   "),
                    c if legal_moves.contains(&(7 - y as u8, x as u8)) => print!("| {} ", c.to_string().red()),
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
        for (position, piece) in self.pieces.iter() {
            board[position.0 as usize][position.1 as usize] = piece.print();
        }
        board
    }

    pub fn move_piece(&mut self, origin: (u8, u8), target: (u8, u8)) {
        let mut moving_piece = self.pieces.remove(&origin).unwrap();
        moving_piece.move_piece(target);
        self.pieces.remove(&target);
        self.pieces.insert(target, moving_piece);
    }
}