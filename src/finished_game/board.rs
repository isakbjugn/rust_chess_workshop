use std::collections::{HashMap, HashSet};

use crate::finished_game::board_contract::BoardContract;
use colored::Colorize;

use crate::finished_game::color::Color;
use crate::finished_game::piece::bishop::Bishop;
use crate::finished_game::piece::king::King;
use crate::finished_game::piece::knight::Knight;
use crate::finished_game::piece::pawn::Pawn;
use crate::finished_game::piece::queen::Queen;
use crate::finished_game::piece::rook::Rook;
use crate::finished_game::piece::Piece;

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
            pieces.push(Box::new(Rook::new(     color, (0, officer_rank))));
            pieces.push(Box::new(Knight::new(   color, (1, officer_rank))));
            pieces.push(Box::new(Bishop::new(   color, (2, officer_rank))));
            pieces.push(Box::new(Queen::new(    color, (3, officer_rank))));
            pieces.push(Box::new(King::new(     color, (4, officer_rank))));
            pieces.push(Box::new(Bishop::new(   color, (5, officer_rank))));
            pieces.push(Box::new(Knight::new(   color, (6, officer_rank))));
            pieces.push(Box::new(Rook::new(     color, (7, officer_rank))));
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
        let moves = piece.get_moves(&team, &rival_team);
        moves
            .into_iter()
            .filter(|&square| {
                let mut new_board = Board {
                    pieces: self.pieces.clone(),
                };
                new_board.move_piece(piece.get_position(), square);
                !new_board.is_check(color)
            })
            .collect()
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

    /// Returns true if the king of specified color is under attack
    fn is_check(&self, color: Color) -> bool {
        let king_position = self.get_king_position(color);
        let team = self.get_positions(color);
        let rival_team = self.get_positions(color.opposite());

        for piece in self.get_pieces_iter(color.opposite()) {
            if piece.get_moves(&rival_team, &team).contains(king_position) {
                return true;
            }
        }
        false
    }

    fn is_checkmate(&self, color: Color) -> bool {
        !self.get_positions(color).iter().any(|pos| !self.get_legal_squares(pos).is_empty())
    }

    fn print(&self, legal_squares: Option<&HashSet<(u8, u8)>>) {
        let board = self.create_board();
        let empty_hashset = HashSet::new();
        let legal_squares = legal_squares.unwrap_or(&empty_hashset);
        let checked_king = self.get_checked_king();

        println!("   {:_<33}", "");
        for (y, row) in board.iter().rev().enumerate() {
            print!("{}  ", 8 - y);
            for (x, piece) in row.iter().enumerate() {
                match *piece {
                    '_' if legal_squares.contains(&(x as u8, 7 - y as u8)) => print!("| {} ", "□".green()),
                    '_' => print!("|   "),
                    c if checked_king == Some(&(x as u8, 7 - y as u8)) => print!("| {} ", c.to_string().red()),
                    c if legal_squares.contains(&(x as u8, 7 - y as u8)) => print!("| {} ", c.to_string().red()),
                    c => print!("| {} ", c),
                }
            }
            println!("|")
        }
        println!("   {:͞<33}", ""); // \u{035E}
        println!("     A   B   C   D   E   F   G   H");
    }
}

impl Board {
    fn get_king_position(&self, color: Color) -> &(u8, u8) {
        self.pieces
            .values()
            .find(|piece| piece.get_color() == color && piece.get_type() == "King")
            .unwrap()
            .get_position()
    }

    fn get_pieces_iter(&self, color: Color) -> impl Iterator<Item = &Box<dyn Piece>> {
        self.pieces.values().filter(move |piece| piece.get_color() == color)
    }

    pub fn get_checked_king(&self) -> Option<&(u8, u8)> {
        for color in [Color::White, Color::Black] {
            if self.is_check(color) {
                return Some(self.get_king_position(color));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::read_to_string;

    use crate::finished_game::board::Board;
    use crate::finished_game::board_contract::BoardContract;
    use crate::finished_game::color::Color;
    use crate::square::{Square, Squares};
    use crate::{assert_eq_set, set};

    impl Board {
        pub fn do_move(&mut self, position: &str, target: &str) {
            let position = position.as_u8().unwrap();
            let target = target.as_u8().unwrap();
            self.move_piece(&position, target);
        }

        pub fn do_moves(&mut self, moves: Vec<&str>) {
            let valid_moves: Vec<_> = moves.into_iter().filter(|&m| m != "x").collect();
            if valid_moves.len() % 2 != 0 {
                panic!("Må oppgi et partall antall posisjoner")
            }
            for move_idx in (0..valid_moves.len()).step_by(2) {
                self.do_move(valid_moves[move_idx], valid_moves[move_idx + 1])
            }
        }
    }

    #[test]
    fn black_pawn_must_block_queen() {
        let mut board = Board::new();
        board.do_move("f7", "f5");
        board.do_move("d1", "h5");
        let legal_moves = set!["g6"];
        assert_eq!(board.get_legal_squares(&"g7".as_u8().unwrap()), legal_moves)
    }

    #[test]
    fn black_pawn_is_pinned() {
        let mut board = Board::new();
        board.do_move("f7", "f5");
        board.do_move("d1", "h5");
        board.do_move("g7", "g6");
        let legal_moves = set!["h5"];
        assert_eq!(board.get_legal_squares(&"g6".as_u8().unwrap()), legal_moves)
    }

    #[test]
    fn pawn_has_two_opening_moves() {
        let board = Board::new();
        let legal_moves = set!["e3", "e4"];
        assert_eq!(board.get_legal_squares(&"e2".as_u8().unwrap()), legal_moves)
    }

    #[test]
    fn white_rook_has_valid_moves() {
        let mut board = Board::new();
        board.do_move("a1", "d4");
        let legal_squares = set!["d3", "d5", "d6", "d7", "a4", "b4", "c4", "e4", "f4", "g4", "h4"];
        assert_eq_set!(board.get_legal_squares(&"d4".as_u8().unwrap()), legal_squares)
    }

    #[test]
    fn no_legal_moves_after_scholars_mate() {
        let mut board = Board::new();
        let moves = read_to_string("games/scholars_mate.txt").unwrap();
        board.do_moves(moves.split_whitespace().collect());
        assert!(board.is_checkmate(Color::Black));
    }
}
