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
        vec![vec!['_'; 8]; 8]
    }

    pub fn print(&self, _: Option<&HashSet<(u8, u8)>>) {
        let board = self.create_board();

        println!("   {:_<33}", "");
        for (y, row) in board.iter().rev().enumerate() {
            print!("{}  ", 8 - y);
            for _ in row.iter() {
                print!("|   ")
            }
            println!("|")
        }
        println!("   {:-<33}", "");
        println!("     A   B   C   D   E   F   G   H");
    }
}
