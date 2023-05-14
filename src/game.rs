use std::collections::HashSet;
use std::io;
use std::io::Write;
use crate::chess_board::ChessBoard;
use crate::board::Board;
use crate::enums::Color;
use crate::squares::Square;

struct Game {
    board: Board,
    turn: Color,
}

impl Game {
    fn new() -> Self {
        Game { board: Board::new(), turn: Color::White }
    }

    fn play(&mut self) {
        self.board.print(None);
        self.print_turn();
        loop {
            let position = self.get_piece();
            let legal_squares = self.board.get_legal_squares(&position);
            if legal_squares.is_empty() {
                println!("Inga lovlege trekk for denne brikka!");
                continue;
            }
            self.board.print(Some(&legal_squares));

            // maybe change this to normal if else block?
            match self.get_move(&position, legal_squares) {
                square if square == position => {
                    println!("Du satte brikka tilbake.");
                    self.board.print(None);
                    continue
                }
                square if self.board.get_square_color(&square) == Some(self.turn.opposite()) => {
                    self.board.capture(&position, square);
                }
                square => {
                    self.board.move_piece(&position, square);
                }
            }

            self.board.print(None);
            self.next_turn();
            self.print_turn();
            self.warn_check();
        }
    }

    fn next_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    fn print_turn(&self) {
        println!("{} sin tur", self.turn.print_capitalised())
    }

    fn warn_check(&self) {
        if self.board.is_check(self.turn) {
            println!("{} konge står i sjakk!", self.turn.print_capitalised());
        }
    }

    fn get_piece(&self) -> (u8, u8) {
        loop {
            print!("Vel ei brikke å flytte: ");
            io::stdout().flush().unwrap();
            if let Some(position) = select_square() {
                match self.board.get_square_color(&position) {
                    Some(color) if color == self.turn => {
                        return position;
                    },
                    Some(_) => {
                        println!("Du valde {}, men det er {} sin tur", self.turn.opposite(), self.turn);
                    },
                    None => {
                        println!("Det er inga brikke i feltet du valde");
                    }
                }
            }
        }
    }

    fn get_move(&self, position: &(u8, u8), mut legal_squares: HashSet<(u8, u8)>) -> (u8, u8) {
        loop {
            print!("Vel eit felt å flytte til: ");
            // Add the actual pieces own position as a legal move, as this means you unselect it
            legal_squares.insert(*position);
            io::stdout().flush().unwrap();
            match select_square() {
                Some(square) if legal_squares.contains(&square) => {
                    return square
                },
                Some(_) => {
                    println!("Feltet du valte er ikkje lov å flytte til!")
                },
                _ => continue
            }
        }
    }
}

/// Read chess square name from stdin and return position
/// For example `"a8" -> (0, 0)`
fn select_square() -> Option<(u8, u8)> {
    let mut square = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut square).unwrap();
    while square.ends_with('\n') || square.ends_with('\r') {
        square.pop();
    }
    square.as_str().as_u8()
}

pub fn main() {
    let mut game = Game::new();
    game.play();
}