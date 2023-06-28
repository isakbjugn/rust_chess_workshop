use std::collections::{HashMap, HashSet};

use colored::Colorize;

use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::color::Color;
use crate::finished_game::piece::bishop::Bishop;
use crate::finished_game::piece::king::{King, KING_NAME};
use crate::finished_game::piece::knight::Knight;
use crate::finished_game::piece::pawn::Pawn;
use crate::finished_game::piece::Piece;
use crate::finished_game::piece::queen::Queen;
use crate::finished_game::piece::rook::Rook;
use crate::square::Square;

pub struct Board {
    pieces: HashMap<(u8, u8), Box<dyn Piece>>,
}

impl BoardContract for Board {
    fn new() -> Board {
        let mut pieces = Vec::<Box<dyn Piece>>::new();
        let teams: Vec<(Color, u8, u8)> = vec![(Color::White, 0, 1), (Color::Black, 7, 6)];
        for &(color, officer_rank, pawn_rank) in &teams {
            for file in 0..=7 {
                pieces.push(Box::new(Pawn::new(color, (file, pawn_rank))));
            }
            pieces.push(Box::new(Rook   ::new(color, (0, officer_rank))));
            pieces.push(Box::new(Knight ::new(color, (1, officer_rank))));
            pieces.push(Box::new(Bishop ::new(color, (2, officer_rank))));
            pieces.push(Box::new(Queen  ::new(color, (3, officer_rank))));
            pieces.push(Box::new(King   ::new(color, (4, officer_rank))));
            pieces.push(Box::new(Bishop ::new(color, (5, officer_rank))));
            pieces.push(Box::new(Knight ::new(color, (6, officer_rank))));
            pieces.push(Box::new(Rook   ::new(color, (7, officer_rank))));
        }
        Board {
            pieces: pieces.into_iter().map(|piece| (*piece.get_position(), piece)).collect()
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
        piece.get_moves(&team, &rival_team)
        // todo!() ikke lov å sette seg selv i sjakk
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

    fn capture(&mut self, position: &(u8, u8), target_square: (u8, u8)) {
        println!("{} fra {} fangar {} på {}", self.get_piece_name(position), position.as_string(), self.get_piece_name(&target_square), target_square.as_string());
        self.move_piece(position, target_square);
    }

    fn get_positions(&self, color: Color) -> HashSet<(u8, u8)> {
        self.pieces.iter()
            .filter_map(|(&position, piece)| if piece.get_color() == color { Some(position) } else { None })
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
                    c => print!("| {} ", c)
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
        self.pieces.values().find(|piece| {
            piece.get_color() == color && piece.get_name() == KING_NAME
        }).unwrap().get_position()
    }

    fn get_pieces_iter(&self, color: Color) -> impl Iterator<Item=&Box<dyn Piece>> {
        self.pieces.values().filter(move |piece| piece.get_color() == color)
    }

    pub fn get_checked_king(&self) -> Option<&(u8, u8)> {
        for color in [Color::White, Color::Black] {
            if self.is_check(color) {
                return Some(self.get_king_position(color))
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::finished_game::board_contract::BoardContract;
    use crate::set;
    use crate::square::{Square, Squares};
    use crate::task_11::board::Board;

    impl Board {
        pub fn do_move(&mut self, position: &str, target: &str) {
            let position = position.as_u8().unwrap();
            let target = target.as_u8().unwrap();
            self.move_piece(&position, target);
        }
    }

    #[test]
    fn black_pawn_on_g7_must_block_queen_as_only_legal_move() {
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
}