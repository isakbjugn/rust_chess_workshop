use std::collections::{HashMap, HashSet};
use crate::pieces::Piece;
use crate::chess_board::ChessBoard;
use crate::enums::{Color, PieceType};
#[cfg(feature = "gui")]
use egui_extras::RetainedImage;

pub struct Board {
    #[cfg(feature = "gui")]
    pub chess_board_image: Option<RetainedImage>,
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
        Board {
            #[cfg(feature = "gui")]
            chess_board_image: Some(RetainedImage::from_image_bytes(
                "chess_board",
                include_bytes!("../assets/board-384-brown.png"),
            ).unwrap()),
            pieces: pieces.into_iter().map(|piece| (piece.position, piece)).collect(),
        }
    }

    fn empty() -> Self where Self: Sized {
        Board {
            #[cfg(feature = "gui")]
            chess_board_image: None,
            pieces: HashMap::<(u8, u8), Piece>::new()
        }
    }

    fn get_piece_name(&self, position: &(u8, u8)) -> String {
        format!("{}", self.pieces.get(position).map(|piece| piece.piece_type).unwrap())
    }

    fn get_square_color(&self, position: &(u8, u8)) -> Option<Color> {
        self.pieces.get(position).map(|piece| piece.color)
    }

    fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)> {
        let color = self.get_square_color(position).unwrap();
        let piece = self.pieces.get(position).unwrap();
        let moves = piece.get_moves(&self);
        moves
            .into_iter()
            .filter(|square| {
                let mut new_board = Board {
                    #[cfg(feature = "gui")]
                    chess_board_image: None,
                    pieces: HashMap::from_iter(self.pieces.clone()) };
                new_board.move_piece(&piece.get_position(), *square);
                !new_board.is_check(color)
            }).collect()
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

    /// Returns true if the king of specified color is under attack
    fn is_check(&self, color: Color) -> bool {
        let king_position = self.pieces.values().find(|piece| {
            piece.get_piece_type() == PieceType::King
        }).unwrap().get_position();

        for piece in self.pieces.values() {
            if piece.get_color() != color && piece.get_moves(self).contains(&king_position) {
                return true
            }
        }
        false
    }
}
