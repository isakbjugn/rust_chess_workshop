use std::collections::{HashMap, HashSet};
use crate::chess_board::ChessBoard;
use crate::pieces_trait::{Piece, Pawn, Rook, Knight, Bishop, Queen, King, KING_NAME};
use crate::enums::Color;
use crate::squares::Square;

pub struct Board {
    pieces: HashMap<(u8, u8), Box<dyn Piece>>,
}

impl ChessBoard for Board {
    fn new() -> Board {
        let mut pieces = Vec::<Box<dyn Piece>>::new();
        let teams: Vec<(Color, u8, u8)> = vec![(Color::White, 0, 1), (Color::Black, 7, 6)];
        for &(color, officer_row, pawn_row) in &teams {
            for col in 0..=7 {
                pieces.push(Box::new(Pawn::new(color, (pawn_row, col))));
            }
            pieces.push(Box::new(Rook::new(color, (officer_row, 0))));
            pieces.push(Box::new(Knight::new(color, (officer_row, 1))));
            pieces.push(Box::new(Bishop::new(color, (officer_row, 2))));
            pieces.push(Box::new(Queen::new(color, (officer_row, 3))));
            pieces.push(Box::new(King::new(color, (officer_row, 4))));
            pieces.push(Box::new(Bishop::new(color, (officer_row, 5))));
            pieces.push(Box::new(Knight::new(color, (officer_row, 6))));
            pieces.push(Box::new(Rook::new(color, (officer_row, 7))));
        }
        Board {
            pieces: pieces.into_iter().map(|piece| (piece.get_position(), piece)).collect()
        }
    }

    fn get_piece_name(&self, position: &(u8, u8)) -> String {
        self.pieces.get(position).map(|piece| piece.get_name()).unwrap()
    }

    fn get_square_color(&self, position: &(u8, u8)) -> Option<Color> {
        self.pieces.get(position).map(|piece| piece.get_color())
    }

    fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)> {
        let color = self.get_square_color(position).expect("Inga brikke på vald posisjon");
        let team = self.get_positions(color);
        let rival_team = self.get_positions(color.opposite());
        let piece = self.pieces.get(position).expect("Inga brikke på vald posisjon.");
        let moves = piece.get_moves(&team, &rival_team);
        moves
            .into_iter()
            .filter(|&square| {
                let mut new_board = Board {
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

    /// Move piece at `position` to square with position `target_square`
    fn move_piece(&mut self, position: &(u8, u8), target_square: (u8, u8)) {
        let mut moving_piece = self.pieces.remove(position).unwrap();
        moving_piece.move_piece(target_square);
        self.pieces.remove(&target_square);
        self.pieces.insert(target_square, moving_piece);
    }

    fn capture(&mut self, position: &(u8, u8), target_square: (u8, u8)) {
        println!("{} fra {} fangar {} på {}", self.get_piece_name(&position), position.as_string(), self.get_piece_name(&target_square), target_square.as_string());
        self.move_piece(position, target_square);
    }

    /// Returns true if the king of specified color is under attack
    fn is_check(&self, color: Color) -> bool {
        let king_position = self.pieces.values().find(|piece| {
            piece.get_color() == color && piece.get_name() == KING_NAME
        }).unwrap().get_position();
        let team = self.get_positions(color);
        let rival_team = self.get_positions(color.opposite());

        for piece in self.get_pieces_iter(color.opposite()) {
            if piece.get_moves(&rival_team, &team).contains(&king_position) {
                return true;
            }
        }
        false
    }
}

impl Board {
    pub fn do_move(&mut self, position: &str, target: &str) {
        let position = position.as_u8().unwrap();
        let target = target.as_u8().unwrap();
        self.move_piece(&position, target);
    }
    fn get_positions(&self, color: Color) -> HashSet<(u8, u8)> {
        self.pieces.iter()
            .filter_map(|(&position, piece)| if piece.get_color() == color { Some(position) } else { None })
            .collect()
    }
    fn get_pieces_iter(&self, color: Color) -> impl Iterator<Item=&Box<dyn Piece>> {
        self.pieces.values().filter(move |piece| piece.get_color() == color)
    }
}

#[cfg(test)]
mod tests {
    use crate::board_trait::Board;
    use crate::chess_board::ChessBoard;
    use crate::squares::{Square, Squares};

    #[test]
    fn black_pawn_must_block_queen() {
        let mut board = Board::new();
        board.do_move("f7", "f5");
        board.do_move("d1", "h5");
        let legal_moves = ["g6"].as_board_positions();
        assert_eq!(board.get_legal_squares(&"g7".as_u8().unwrap()), legal_moves)
    }

    #[test]
    fn black_pawn_is_pinned() {
        let mut board = Board::new();
        board.do_move("f7", "f5");
        board.do_move("d1", "h5");
        board.do_move("g7", "g6");
        let legal_moves = ["h5"].as_board_positions();
        assert_eq!(board.get_legal_squares(&"g6".as_u8().unwrap()), legal_moves)
    }

    #[test]
    fn pawn_has_two_opening_moves() {
        let board = Board::new();
        let legal_moves = ["e3", "e4"].as_board_positions();
        assert_eq!(board.get_legal_squares(&"e2".as_u8().unwrap()), legal_moves)
    }

    #[test]
    fn white_rook_has_valid_moves() {
        let mut board = Board::new();
        board.do_move("a1", "d4");
        let legal_squares = ["d3", "d5", "d6", "d7", "a4", "b4", "c4", "e4", "f4", "g4", "h4"].as_board_positions();
        assert_eq!(board.get_legal_squares(&"d4".as_u8().unwrap()), legal_squares)
    }
}
