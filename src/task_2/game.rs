use std::collections::HashSet;
use std::io;
use std::io::{BufRead, Write};

use crate::finished_game::color::Color;
use crate::square::Square;
use crate::task_2::board::Board;

struct Game {
    board: Board,
    turn: Color,
    finished: bool,
}

impl Game {
    fn new() -> Self {
        Game { board: Board::new(), turn: Color::White, finished: false }
    }

    fn play(&mut self, input: &mut impl BufRead) {
        self.board.print(None);
        self.print_turn();
        loop {
            let Some(position) = self.get_piece(input) else { break; };
            let legal_squares = self.board.get_legal_squares(&position);
            if legal_squares.is_empty() {
                println!("Inga lovlege trekk for denne brikka!");
                continue;
            }
            self.board.print(Some(&legal_squares));

            // maybe change this to normal if else block?
            let Some(position_to_move_to) = self.get_move(&position, legal_squares, input) else { break };
            match position_to_move_to {
                position_to_move_to if position_to_move_to == position => {
                    println!("Du satte brikka tilbake.");
                    self.board.print(None);
                    continue
                }
                position_to_move_to if self.board.get_square_color(&position_to_move_to) == Some(self.turn.opposite()) => {
                    self.board.capture(&position, position_to_move_to);
                }
                position_to_move_to => {
                    self.board.move_piece(&position, position_to_move_to);
                }
            }

            self.board.print(None);
            self.next_turn();
            self.print_turn();
        }
    }

    fn next_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    fn print_turn(&self) {
        println!("{} sin tur", self.turn.print_capitalised())
    }

    fn get_piece(&mut self, input: &mut impl BufRead) -> Option<(u8, u8)> {
        while !self.finished {
            print!("Vel ei brikke å flytte: ");
            io::stdout().flush().unwrap();
            if let Some(position) = self.select_square(input) {
                match self.board.get_square_color(&position) {
                    Some(color) if color == self.turn => {
                        return Some(position);
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
        None
    }

    fn get_move(&mut self, position: &(u8, u8), mut legal_squares: HashSet<(u8, u8)>, input: &mut impl BufRead) -> Option<(u8, u8)> {
        while !self.finished {
            print!("Vel eit felt å flytte til: ");
            // Add the actual pieces own position as a legal move, as this means you unselect it
            legal_squares.insert(*position);
            io::stdout().flush().unwrap();
            match self.select_square(input) {
                Some(square) if legal_squares.contains(&square) => {
                    return Some(square)
                },
                Some(_) => {
                    println!("Feltet du valte er ikkje lov å flytte til!")
                },
                _ => continue
            }
        }
        None
    }

    /// Read chess square name from stdin and return position
    /// For example `"a8" -> (0, 0)`
    fn select_square(&mut self, input: &mut impl BufRead) -> Option<(u8, u8)> {
        let mut square = String::new();
        input.read_line(&mut square).unwrap();
        square.retain(|c| !c.is_ascii_whitespace());

        if square == "x" {
            self.exit_game();
            return None
        }

        square.as_str().as_u8()
    }

    pub fn exit_game(&mut self) {
        self.finished = true;
    }
}

pub fn main() {
    let mut game = Game::new();
    game.play(&mut io::stdin().lock());
}