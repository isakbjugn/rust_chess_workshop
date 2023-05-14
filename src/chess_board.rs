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
    fn get_king_position(&self, color: Color) -> &(u8, u8);

    fn get_checked_kings(&self) -> Vec<&(u8, u8)> {
        let mut checked_kings = Vec::new();
        for color in [Color::White, Color::Black] {
            if self.is_check(color) {
                checked_kings.push(self.get_king_position(color))
            }
        }
        checked_kings
    }

    fn print(&self, legal_squares: Option<&HashSet<(u8, u8)>>) {
        let board = self.create_board();
        let empty_hashset = HashSet::new();
        let legal_squares = legal_squares.unwrap_or(&empty_hashset);
        let checked_kings = self.get_checked_kings();

        println!("   {:_<33}", "");
        for (y, row) in board.iter().rev().enumerate() {
            print!("{}  ", 8 - y);
            for (x, piece) in row.iter().enumerate() {
                match *piece {
                    '_' if legal_squares.contains(&(7 - y as u8, x as u8)) => print!("| {} ", "□".green()),
                    '_' => print!("|   "),
                    c if checked_kings.contains(&&(7 - y as u8, x as u8)) => print!("| {} ", c.to_string().red()),
                    c if legal_squares.contains(&(7 - y as u8, x as u8)) => print!("| {} ", c.to_string().magenta()),
                    c => print!("| {} ", c)
                }
            }
            println!("|")
        }
        println!("   {:͞<33}", ""); // \u{035E}
        println!("     A   B   C   D   E   F   G   H");
    }
}