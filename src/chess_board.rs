use std::io;
use std::io::Write;
use crate::pieces::{Piece, Color, Type};

struct Board {
    board: Vec<Vec<char>>
}

impl Board {
    fn new(pieces: &[Piece]) -> Board {
        let mut board = vec![vec!['x'; 8]; 8];
        for piece in pieces.iter() {
            board[piece.position.0 as usize][piece.position.1 as usize] = piece.print_piece();
        }
        Board { board }
    }
    fn position_holds_piece(&self, position: (u8, u8)) -> bool {
        self.board[position.0 as usize][position.1 as usize] != 'x'
    }
}

struct Game {
    pieces: Vec<Piece>,
    board: Board
}

impl Game {
    fn new() -> Game {
        let mut pieces = Vec::<Piece>::new();
        let teams: Vec<(Color, u8, u8)> = vec![(Color::White, 0, 1), (Color::Black, 7, 6)];
        for team in teams.iter() {
            for col in 0..=7 {
                pieces.push(Piece {color: team.0, piece_type: Type::Pawn, position: (team.2, col)})
            }
            pieces.push(Piece {color: team.0, piece_type: Type::Rook, position: (team.1, 0)});
            pieces.push(Piece {color: team.0, piece_type: Type::Rook, position: (team.1, 7)});
            pieces.push(Piece {color: team.0, piece_type: Type::Knight, position: (team.1, 1)});
            pieces.push(Piece {color: team.0, piece_type: Type::Knight, position: (team.1, 6)});
            pieces.push(Piece {color: team.0, piece_type: Type::Bishop, position: (team.1, 2)});
            pieces.push(Piece {color: team.0, piece_type: Type::Bishop, position: (team.1, 5)});
            pieces.push(Piece {color: team.0, piece_type: Type::Queen, position: (team.1, 3)});
            pieces.push(Piece {color: team.0, piece_type: Type::King, position: (team.1, 4)});
        }
        let board = Board::new(&pieces);

        Game { pieces, board }
    }

    fn select_piece(&mut self, position: (u8, u8)) -> Option<&mut Piece> {
        //println!("{:?}", position);
        //println!("{:#?}", self.pieces);
        match self.pieces.iter().position(|piece| piece.position == position) {
            Some(idx) => Some(&mut self.pieces[idx]),
            None => None
        }
    }

    fn update_board(&mut self) {
        self.board = Board::new(&self.pieces)
    }

    fn draw_board(&mut self) {
        self.update_board();
        println!("   {:_<33}", "");
        for (row_idx, row) in self.board.board.iter().rev().enumerate() {
            print!("{}  ", 8 - row_idx);
            for piece in row {
                match *piece {
                    'x' => print!("|   "),
                    x => print!("| {} ", x)
                }
            }
            println!("|")
        }
        println!("   {:\u{035E}<33}", "");
        println!("     A   B   C   D   E   F   G   H");
    }
}

pub fn main() {
    let mut game = Game::new();

    loop {
        game.draw_board();

        print!("Vel ei brikke å flytte: ");
        io::stdout().flush().unwrap();
        match select_square() {
            Some(origin) if game.board.position_holds_piece(origin) => {
                print!("Vel eit felt å flytte til: ");
                io::stdout().flush().unwrap();
                match select_square() {
                    Some(square) => {
                        if game.board.position_holds_piece(square) {
                            println!("Feltet er allereie tatt")
                        }
                        println!("Flyttar {} fra {:?} til {:?}", game.board.board[origin.0 as usize][origin.1 as usize], origin, square);
                        let piece = game.select_piece(origin).unwrap();
                        piece.move_piece(square);
                    },
                    _ => continue
                }
            },
            _ => continue
        }
    }
}

fn select_square() -> Option<(u8, u8)> {
    let mut square = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut square).unwrap();
    while square.ends_with('\n') || square.ends_with('\r') {
        square.pop();
    }
    square_name_to_position(&square[..])
}

fn square_name_to_position(square: &str) -> Option<(u8, u8)> {
    if square.chars().count() != 2 {return None}
    let mut chars = square.chars();
    let col = chars.next().unwrap().to_ascii_lowercase() as u8 - 97;
    let row = chars.next().unwrap().to_digit(10).unwrap() as u8 - 1;

    if col < 8 && row < 8 {
        return Some((row, col));
    }
    None
}
