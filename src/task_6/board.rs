use std::collections::{HashMap, HashSet};
use colored::Colorize;
use crate::color::Color;
use crate::task_6::piece::pawn::Pawn;
use crate::task_6::piece::knight::Knight;
use crate::task_6::piece::Piece;

pub struct Board {
    pieces: HashMap<(u8, u8), Box<dyn Piece>>,
}

impl Board {
    pub fn new() -> Board {
        let mut pieces = Vec::<Box<dyn Piece>>::new();
        let teams: Vec<(Color, u8, u8)> = vec![(Color::White, 0, 1), (Color::Black, 7, 6)];
        for &(color, officer_rank, pawn_rank) in &teams {
            for col in 0..=7 {
                pieces.push(Box::new(Pawn::new(color, (pawn_rank, col))));
                pieces.push(Box::new(Knight::new(   color, (1, officer_rank))));
                pieces.push(Box::new(Knight::new(   color, (6, officer_rank))));
            }
        }
        Board {
            pieces: pieces.into_iter().map(|piece| (*piece.get_position(), piece)).collect()
        }
    }

    pub fn get_square_color(&self, position: &(u8, u8)) -> Option<Color> {
        self.pieces.get(position).map(|piece| piece.get_color())
    }

    pub fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)> {
        let color = self.get_square_color(position).expect("Inga brikke på vald posisjon");
        let team = self.get_positions(color);
        let rival_team = self.get_positions(color.opposite());
        let piece = self.pieces.get(position).expect("Inga brikke på vald posisjon.");
        piece.get_moves(&team, &rival_team)
    }

    fn create_board(&self) -> Vec<Vec<char>> {
        let mut board = vec![vec!['_'; 8]; 8];
        for (position, piece) in &self.pieces {
            board[position.0 as usize][position.1 as usize] = piece.print();
        }
        board
    }

    /// Move piece at `position` to square with position `target_square`
    pub fn move_piece(&mut self, position: &(u8, u8), target_square: (u8, u8)) {
        let mut moving_piece = self.pieces.remove(position).unwrap();
        moving_piece.move_piece(target_square);
        self.pieces.remove(&target_square);
        self.pieces.insert(target_square, moving_piece);
    }

    pub fn capture(&mut self, position: &(u8, u8), target_square: (u8, u8)) {
        todo!()
    }

    fn get_positions(&self, color: Color) -> HashSet<(u8, u8)> {
        self.pieces.iter()
            .filter_map(|(&position, piece)| if piece.get_color() == color { Some(position) } else { None })
            .collect()
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
        println!("   {:͞<33}", ""); // \u{035E}
        println!("     A   B   C   D   E   F   G   H");
    }
}
