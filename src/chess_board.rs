use std::collections::HashSet;
use colored::Colorize;
use crate::enums::Color;

pub trait ChessBoard {
    fn new() -> Self where Self: Sized;
    fn get_piece_name(&self, position: &(u8, u8)) -> String;
    fn get_square_color(&self, position: &(u8, u8)) -> Option<Color>;
    fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)>;
    fn create_board(&self) -> Vec<Vec<char>>;
    fn move_piece(&mut self, position: &(u8, u8), square: (u8, u8));
    fn capture(&mut self, position: &(u8, u8), square: (u8, u8));
    fn is_check(&self, color: Color) -> bool;

    /// Filter moves which are blocked by other pieces, ensuring pieces such as the rook cannot move
    /// through other pieces
    fn filter_move_directions(&self, move_directions: &HashSet<Vec<(u8, u8)>>, color: Color) -> HashSet<(u8, u8)> {
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

    fn filter_out_same_color(&self, moves: HashSet<(u8, u8)>, color: Color) -> HashSet<(u8, u8)> {
        moves.iter().cloned()
            .filter(|square| self.get_square_color(square) != Some(color))
            .collect()
    }

    fn print(&self, legal_squares: Option<&HashSet<(u8, u8)>>) {
        let board = self.create_board();
        let empty_hashset = HashSet::new();
        let legal_squares = legal_squares.unwrap_or(&empty_hashset);

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
}