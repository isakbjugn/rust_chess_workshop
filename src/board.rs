use std::collections::{HashMap, HashSet};
use crate::pieces::Piece;
use crate::chess_board::ChessBoard;
use crate::enums::{Color, PieceType};

pub struct Board {
    pieces: HashMap<(u8, u8), Piece>
}

impl ChessBoard for Board {
    fn new() -> Board {
        let mut pieces = Vec::<Piece>::new();
        let teams: Vec<(Color, u8, u8)> = vec![(Color::White, 0, 1), (Color::Black, 7, 6)];
        for team in &teams {
            for col in 0..=7 {
                pieces.push(Piece::new(team.0, PieceType::Pawn, (team.2, col)));
            }
            pieces.push(Piece::new(team.0, PieceType::Rook, (team.1, 0)));
            pieces.push(Piece::new(team.0, PieceType::Rook, (team.1, 7)));
            pieces.push(Piece::new(team.0, PieceType::Knight, (team.1, 1)));
            pieces.push(Piece::new(team.0, PieceType::Knight, (team.1, 6)));
            pieces.push(Piece::new(team.0, PieceType::Bishop, (team.1, 2)));
            pieces.push(Piece::new(team.0, PieceType::Bishop, (team.1, 5)));
            pieces.push(Piece::new(team.0, PieceType::Queen, (team.1, 3)));
            pieces.push(Piece::new(team.0, PieceType::King, (team.1, 4)));
        }
        Board { pieces: pieces.into_iter().map(|piece| (piece.position, piece)).collect() }
    }

    fn empty() -> Self where Self: Sized {
        Board { pieces: HashMap::<(u8, u8), Piece>::new() }
    }

    fn get_piece_name(&self, position: &(u8, u8)) -> String {
        format!("{}", self.pieces.get(position).map(|piece| piece.piece_type).unwrap())
    }

    fn get_square_color(&self, position: &(u8, u8)) -> Option<Color> {
        self.pieces.get(position).map(|piece| piece.color)
    }

    fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)> {
        self.pieces.get(position).unwrap().get_moves(&self)
    }

    fn capture(&mut self, position: &(u8, u8), square: (u8, u8)) {
        println!("{} fra {:?} fangar {} på {:?}", self.get_piece_name(&position), position, self.get_piece_name(&square), square);
        self.move_piece(position, square);
    }

    fn create_board(&self) -> Vec<Vec<char>> {
        let mut board = vec![vec!['_'; 8]; 8];
        for (position, piece) in &self.pieces {
            board[position.0 as usize][position.1 as usize] = piece.print();
        }
        board
    }

    fn move_piece(&mut self, position: &(u8, u8), square: (u8, u8)) {
        let mut moving_piece = self.pieces.remove(position).unwrap();
        moving_piece.move_piece(square);
        self.pieces.remove(&square);
        self.pieces.insert(square, moving_piece);
    }
}

impl Board {
    fn _can_castle() -> bool {
        // TODO: Implement check if player can castle
        true
    }

    fn _can_en_passant() -> bool {
        // TODO: Implement check if player can perform en passant
        false
    }

    fn get_pawn_capture_moves(&self, position: &(u8, u8)) -> Option<HashSet<(u8, u8)>> {
        // TODO: Add possible en passant captures
        let mut captures = HashSet::new();
        match self.get_square_color(position) {
            Some(Color::White) if position.0 < 7 => {
                let capture_y = position.0 + 1;
                if let Some(Color::Black) = self.get_square_color(&(capture_y, position.1 - 1)) {
                    captures.insert((capture_y, position.1 - 1));
                }
                if let Some(Color::Black) = self.get_square_color(&(capture_y, position.1 + 1)) {
                    captures.insert((capture_y, position.1 + 1));
                }
                Some(captures)
            },
            Some(Color::Black) if position.0 > 0 => {
                let capture_y = position.0 - 1;
                if let Some(Color::White) = self.get_square_color(&(capture_y, position.1 - 1)) {
                    captures.insert((capture_y, position.1 - 1));
                }
                if let Some(Color::White) = self.get_square_color(&(capture_y, position.1 + 1)) {
                    captures.insert((capture_y, position.1 + 1));
                }
                Some(captures)
            },
            _ => None,
        }
    }
}