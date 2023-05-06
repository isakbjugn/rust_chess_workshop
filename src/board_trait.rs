use std::collections::{HashMap, HashSet};
use crate::chess_board::ChessBoard;
use crate::pieces_trait::{Piece, Pawn, Rook, Knight, Bishop, Queen, King, KING_NAME};
#[cfg(feature = "gui")]
use egui_extras::RetainedImage;
use crate::enums::Color;

pub struct Board {
    #[cfg(feature = "gui")]
    pub chess_board_image: Option<RetainedImage>,
    pieces: HashMap<(u8, u8), Box<dyn Piece>>,
}

impl ChessBoard for Board {
    fn new() -> Board {
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
            #[cfg(feature = "gui")]
            chess_board_image: Some(RetainedImage::from_image_bytes(
                "chess_board",
                include_bytes!("../assets/board-384-brown.png"),
            ).unwrap()),
            pieces: pieces.into_iter().map(|piece| (piece.get_position(), piece)).collect()
        }
    }

    fn empty() -> Self where Self: Sized {
        Board {
            #[cfg(feature = "gui")]
            chess_board_image: None,
            pieces: HashMap::<(u8, u8), Box<dyn Piece>>::new()
        }
    }

    fn get_piece_name(&self, position: &(u8, u8)) -> String {
        self.pieces.get(position).map(|piece| piece.get_name()).unwrap()
    }

    fn get_square_color(&self, position: &(u8, u8)) -> Option<Color> {
        self.pieces.get(position).map(|piece| piece.get_color())
    }

    fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)> {
        let color = self.get_square_color(position).unwrap();
        let piece = self.pieces.get(position).unwrap();
        let moves = piece.get_moves(&self);
        moves
            .into_iter()
            .filter(|&square| {
                let mut new_board = Board {
                    #[cfg(feature = "gui")]
                    chess_board_image: None,
                    pieces: self.pieces.clone()
                };
                new_board.move_piece(&piece.get_position(), square);
                !new_board.is_check(color)
            }).collect()
    }

    fn create_board(&self) -> Vec<Vec<char>> {
        let mut board = vec![vec!['_'; 8]; 8];
        for (position, piece) in &self.pieces {
            board[position.0 as usize][position.1 as usize] = piece.print();
        }
        board
    }

    /// Move piece at `position` to square with position `target`
    fn move_piece(&mut self, position: &(u8, u8), target: (u8, u8)) {
        let mut moving_piece = self.pieces.remove(position).unwrap();
        moving_piece.move_piece(target);
        self.pieces.remove(&target);
        self.pieces.insert(target, moving_piece);
    }

    fn capture(&mut self, position: &(u8, u8), square: (u8, u8)) {
        println!("{} fra {:?} fangar {} på {:?}", self.get_piece_name(&position), position, self.get_piece_name(&square), square);
        self.move_piece(position, square);
    }

    /// Returns true if the king of specified color is under attack
    fn is_check(&self, color: Color) -> bool {
        let king_position = self.pieces.values().find(|piece| {
            piece.get_color() == color && piece.get_name() == KING_NAME
        }).unwrap().get_position();

        for piece in self.pieces.values().filter(|p| p.get_color() != color) {
            if piece.get_moves(self).contains(&king_position) {
                return true
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::enums::{Color, PieceType};
    use crate::pieces::Piece;

    #[test]
    fn test() {

    }
}
