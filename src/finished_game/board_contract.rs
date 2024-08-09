#![allow(unused)]
use std::collections::HashSet;
use colored::Colorize;
use crate::finished_game::color::Color;

pub trait BoardContract {
    fn new() -> Self where Self: Sized;
    fn get_piece_type(&self, position: &(u8, u8)) -> &'static str;
    fn get_square_color(&self, position: &(u8, u8)) -> Option<Color>;
    fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)>;
    fn get_castle_moves(&self, king_position: &(u8, u8)) -> HashSet<(u8, u8)> {
        HashSet::new()
        // todo!("Skal implementeres i oppgave 13")
    }
    fn create_board(&self) -> Vec<Vec<char>>;
    fn move_piece(&mut self, position: &(u8, u8), target_square: (u8, u8));
    fn get_positions(&self, color: Color) -> HashSet<(u8, u8)>;
    fn is_check(&self, color: Color) -> bool {
        false
        // todo!("Skal implementeres i oppgave 8")
    }
    fn is_checkmate(&self, color: Color) -> bool {
        false
        // todo!("Skal implementeres i oppgave 9")
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
                    '_' if legal_squares.contains(&(x as u8, 7 - y as u8)) => print!("| {} ", "□".green()),
                    '_' => print!("|   "),
                    c if legal_squares.contains(&(x as u8, 7 - y as u8)) => print!("| {} ", c.to_string().magenta()),
                    c => print!("| {} ", c)
                }
            }
            println!("|")
        }
        println!("   {:͞<33}", ""); // \u{035E}
        println!("     A   B   C   D   E   F   G   H");
    }
}