use std::collections::{HashMap, HashSet};

use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::color::Color;
use crate::finished_game::piece::knight::Knight;
use crate::finished_game::piece::pawn::Pawn;
use crate::finished_game::piece::Piece;
use crate::task_7::piece::king::King;
use crate::task_7::piece::rook::Rook;

pub struct Board {
    pieces: HashMap<(u8, u8), Box<dyn Piece>>,
}

impl BoardContract for Board {
    #[rustfmt::skip]
    fn new() -> Board {
        let mut pieces = Vec::<Box<dyn Piece>>::new();
        let teams: Vec<(Color, u8, u8)> = vec![(Color::White, 0, 1), (Color::Black, 7, 6)];
        for &(color, officer_rank, pawn_rank) in &teams {
            for file in 0..=7 {
                pieces.push(Box::new(Pawn::new(color, (file, pawn_rank))));
            }
            pieces.push(Box::new(Rook   ::new(color, (0, officer_rank))));
            pieces.push(Box::new(Knight ::new(color, (1, officer_rank))));
            pieces.push(Box::new(King   ::new(color, (4, officer_rank))));
            pieces.push(Box::new(Knight ::new(color, (6, officer_rank))));
            pieces.push(Box::new(Rook   ::new(color, (7, officer_rank))));
        }
        Board {
            pieces: pieces.into_iter().map(|piece| (*piece.get_position(), piece)).collect(),
        }
    }

    fn get_piece_type(&self, position: &(u8, u8)) -> &'static str {
        self.pieces.get(position).map(|piece| piece.get_type()).unwrap()
    }

    fn get_square_color(&self, position: &(u8, u8)) -> Option<Color> {
        self.pieces.get(position).map(|piece| piece.get_color())
    }

    fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)> {
        let color = self.get_square_color(position).expect("Inga brikke på vald posisjon");
        let team = self.get_positions(color);
        let rival_team = self.get_positions(color.opposite());
        let piece = self.pieces.get(position).expect("Inga brikke på vald posisjon.");
        piece.get_moves(&team, &rival_team)
    }

    fn create_board(&self) -> Vec<Vec<char>> {
        let mut board = vec![vec!['_'; 8]; 8];
        for (position, piece) in &self.pieces {
            board[position.1 as usize][position.0 as usize] = piece.print();
        }
        board
    }

    /// Move piece at `position` to square with position `target_square`
    fn move_piece(&mut self, position: &(u8, u8), target_square: (u8, u8)) {
        let mut moving_piece = self.pieces.remove(position).unwrap();
        moving_piece.move_piece(target_square);
        self.pieces.remove(&target_square);
        self.pieces.insert(target_square, moving_piece);
    }

    fn get_positions(&self, color: Color) -> HashSet<(u8, u8)> {
        self.pieces
            .iter()
            .filter_map(|(&position, piece)| {
                if piece.get_color() == color {
                    Some(position)
                } else {
                    None
                }
            })
            .collect()
    }
}
