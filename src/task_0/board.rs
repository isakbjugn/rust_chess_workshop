use std::collections::{HashMap, HashSet};
use colored::Colorize;
use crate::square::Square;
use crate::task_1::*;

pub struct Board;

impl Board {
    pub fn new() -> Board {
        Board
    }

    fn create_board(&self) -> Vec<Vec<char>> {
        let mut board = vec![vec!['_'; 8]; 8];
        board
    }

    pub fn print(&self, legal_squares: Option<&HashSet<(u8, u8)>>) {
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
                    c if legal_squares.contains(&(7 - y as u8, x as u8)) => print!("| {} ", c.to_string().magenta()),
                    c => print!("| {} ", c)
                }
            }
            println!("|")
        }
        println!("   {:-<33}", "");
        println!("     A   B   C   D   E   F   G   H");
    }
}
