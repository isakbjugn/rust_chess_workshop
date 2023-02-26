use std::collections::HashMap;
use crate::pieces::{Piece, Color, Type};

pub struct Board {
    pieces: HashMap<(u8, u8), Piece>
}

impl Board {
    pub fn new() -> Board {
        let mut pieces = Vec::<Piece>::new();
        let teams: Vec<(Color, u8, u8)> = vec![(Color::White, 0, 1), (Color::Black, 7, 6)];
        for team in teams.iter() {
            for col in 0..=7 {
                pieces.push(Piece::new(team.0, Type::Pawn, (team.2, col)));
            }
            pieces.push(Piece::new(team.0, Type::Rook, (team.1, 0)));
            pieces.push(Piece::new(team.0, Type::Rook, (team.1, 7)));
            pieces.push(Piece::new(team.0, Type::Knight, (team.1, 1)));
            pieces.push(Piece::new(team.0, Type::Knight, (team.1, 6)));
            pieces.push(Piece::new(team.0, Type::Bishop, (team.1, 2)));
            pieces.push(Piece::new(team.0, Type::Bishop, (team.1, 5)));
            pieces.push(Piece::new(team.0, Type::Queen, (team.1, 3)));
            pieces.push(Piece::new(team.0, Type::King, (team.1, 4)));
        }
        Board { pieces: pieces.iter().map(|&piece| (piece.position, piece)).collect() }
    }

    pub fn get_square_symbol(&self, position: (u8, u8)) -> Option<Type> {
        self.pieces.get(&position).map(|piece| piece.piece_type)
    }

    pub fn get_square_color(&self, position: (u8, u8)) -> Option<Color> {
        self.pieces.get(&position).map(|piece| piece.color)
    }

    pub fn print(&self) {
        let board = self.create_board();
        println!("   {:_<33}", "");
        for (row_idx, row) in board.iter().rev().enumerate() {
            print!("{}  ", 8 - row_idx);
            for piece in row {
                match *piece {
                    'x' => print!("|   "),
                    x => print!("| {} ", x)
                }
            }
            println!("|")
        }
        println!("   {:\u{035E}<33}", "");
        println!("     A   B   C   D   E   F   G   H");
    }

    fn create_board(&self) -> Vec<Vec<char>> {
        let mut board = vec![vec!['x'; 8]; 8];
        for (position, piece) in self.pieces.iter() {
            board[position.0 as usize][position.1 as usize] = piece.print();
        }
        board
    }

    pub fn move_piece(&mut self, origin: (u8, u8), target: (u8, u8)) {
        let mut moving_piece = *self.pieces.get(&origin).unwrap();
        moving_piece.move_piece(target);
        self.pieces.remove(&origin);
        self.pieces.remove(&target);
        self.pieces.insert(target, moving_piece);
    }
}
