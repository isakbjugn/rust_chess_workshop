use std::collections::{HashMap, HashSet};
use crate::pieces::{Piece, Color, Type};
use crate::utils::to_moves;
use colored::Colorize;

pub struct Board {
    pieces: HashMap<(u8, u8), Piece>
}

impl Board {
    pub fn new() -> Board {
        let mut pieces = Vec::<Piece>::new();
        let teams: Vec<(Color, u8, u8)> = vec![(Color::White, 0, 1), (Color::Black, 7, 6)];
        for team in teams.iter() {
            for col in 0..=7 {
                pieces.push(Piece::new(team.0, Type::Pawn, (team.2, col)));
            }
            pieces.push(Piece::new(team.0, Type::Rook, (team.1, 0)));
            pieces.push(Piece::new(team.0, Type::Rook, (team.1, 7)));
            pieces.push(Piece::new(team.0, Type::Knight, (team.1, 1)));
            pieces.push(Piece::new(team.0, Type::Knight, (team.1, 6)));
            pieces.push(Piece::new(team.0, Type::Bishop, (team.1, 2)));
            pieces.push(Piece::new(team.0, Type::Bishop, (team.1, 5)));
            pieces.push(Piece::new(team.0, Type::Queen, (team.1, 3)));
            pieces.push(Piece::new(team.0, Type::King, (team.1, 4)));
        }
        Board { pieces: pieces.iter().map(|&piece| (piece.position, piece)).collect() }
    }

    pub fn get_square_symbol(&self, position: &(u8, u8)) -> Option<Type> {
        self.pieces.get(position).map(|piece| piece.piece_type)
    }

    pub fn get_square_color(&self, position: &(u8, u8)) -> Option<Color> {
        self.pieces.get(position).map(|piece| piece.color)
    }

    fn _can_castle() -> bool {
        // TODO: Implement check if player can castle
        true
    }

    fn _can_en_passant() -> bool {
        // TODO: Implement check if player can perform en passant
        false
    }

    fn get_pawn_capture_moves(&self, position: &(u8, u8)) -> Option<HashSet<(u8, u8)>> {
        // TODO: Add possible en passant captures
        let mut captures = HashSet::new();
        match self.get_square_color(position) {
            Some(Color::White) if position.0 < 7 => {
                let capture_y = position.0 + 1;
                if let Some(Color::Black) = self.get_square_color(&(capture_y, position.1 - 1)) {
                    captures.insert((capture_y, position.1 - 1));
                }
                if let Some(Color::Black) = self.get_square_color(&(capture_y, position.1 + 1)) {
                    captures.insert((capture_y, position.1 + 1));
                }
                Some(captures)
            },
            Some(Color::Black) if position.0 > 0 => {
                let capture_y = position.0 - 1;
                if let Some(Color::White) = self.get_square_color(&(capture_y, position.1 - 1)) {
                    captures.insert((capture_y, position.1 - 1));
                }
                if let Some(Color::White) = self.get_square_color(&(capture_y, position.1 + 1)) {
                    captures.insert((capture_y, position.1 + 1));
                }
                Some(captures)
            },
            _ => None,
        }
    }

    fn get_moves(&self, position: &(u8, u8)) -> HashSet<(u8, u8)> {
        let legal_moves = self.pieces.get(position).unwrap().get_moves();
        let color = self.get_square_color(position).unwrap();

        match self.pieces.get(position).unwrap().piece_type {
            Type::Knight | Type::King => {
                let mut moves = to_moves(legal_moves);
                moves.retain(|m| self.get_square_color(m) != Some(color));
                moves
            },
            Type::Pawn => {
                let moves = self.get_unblocked_squares(legal_moves, color);
                match self.get_pawn_capture_moves(position) {
                    Some(capture_moves) => moves.union(&capture_moves).cloned().collect(),
                    None => moves,
                }
            }
            _ => self.get_unblocked_squares(legal_moves, color)
        }
    }

    fn get_unblocked_squares(&self, legal_moves: HashSet<Vec<(u8, u8)>>, color: Color) -> HashSet<(u8, u8)> {
        let mut moves = HashSet::new();
        for line in legal_moves {
            'direction: for square in line {
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

    pub fn print(&self) {
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

    pub fn print_with_legal_moves(&self, piece: &(u8, u8)) {
        let board = self.create_board();
        let legal_moves = self.get_moves(piece);

        println!("   {:_<33}", "");
        for (y, row) in board.iter().rev().enumerate() {
            print!("{}  ", 8 - y);
            for (x, piece) in row.iter().enumerate() {
                match *piece {
                    '_' if legal_moves.contains(&(7 - y as u8, x as u8)) => print!("| {} ", "□".green()),
                    '_' => print!("|   "),
                    c if legal_moves.contains(&(7 - y as u8, x as u8)) => print!("| {} ", c.to_string().red()),
                    c => print!("| {} ", c)
                }
            }
            println!("|")
        }
        println!("   {:\u{035E}<33}", "");
        println!("     A   B   C   D   E   F   G   H");
    }

    fn create_board(&self) -> Vec<Vec<char>> {
        let mut board = vec![vec!['_'; 8]; 8];
        for (position, piece) in self.pieces.iter() {
            board[position.0 as usize][position.1 as usize] = piece.print();
        }
        board
    }

    pub fn move_piece(&mut self, origin: (u8, u8), target: (u8, u8)) {
        let mut moving_piece = *self.pieces.get(&origin).unwrap();
        moving_piece.move_piece(target);
        self.pieces.remove(&origin);
        self.pieces.remove(&target);
        self.pieces.insert(target, moving_piece);
    }
}
