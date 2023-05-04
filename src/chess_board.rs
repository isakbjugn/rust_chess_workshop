use std::collections::HashSet;
use colored::Colorize;
use crate::enums::Color;

pub trait ChessBoard {
    fn new() -> Self where Self: Sized;
    fn empty() -> Self where Self: Sized;
    fn get_piece_name(&self, position: &(u8, u8)) -> String;
    fn get_square_color(&self, position: &(u8, u8)) -> Option<Color>;
    fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)>;
    fn create_board(&self) -> Vec<Vec<char>>;
    fn move_piece(&mut self, position: &(u8, u8), square: (u8, u8));
    fn capture(&mut self, position: &(u8, u8), square: (u8, u8));
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
    fn print(&self) {
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
    fn print_with_legal_moves(&self, legal_squares: &HashSet<(u8, u8)>) {
        let board = self.create_board();

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