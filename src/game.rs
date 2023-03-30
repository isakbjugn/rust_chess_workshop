use std::collections::HashSet;
use std::io;
use std::io::Write;
use crate::board_trait::Board;
use crate::enums::Color;
use crate::utils::{select_square};

struct Game {
    board: Board,
    turn: Color,
}

impl Game {
    fn new() -> Self {
        Game { board: Board::new(), turn: Color::White}
    }

    fn play(&mut self) {
        'turn: loop {
            self.board.print();
            match self.turn {
                Color::White => println!("Kvit sin tur"),
                Color::Black => println!("Svart sin tur")
            }
            let position = self.get_piece();
            let legal_squares = self.board.get_legal_squares(&position);
            if legal_squares.is_empty() {
                println!("Inga lovlege trekk for denne brikka!");
                continue
            }
            match self.get_move(&position, legal_squares) {
                s if s == position => {
                    println!("Du satte brikka tilbake.");
                    continue
                },
                s if self.board.get_square_color(&s) == Some(self.get_opponent_color()) => {
                    self.board.capture(&position, s);
                }
                s => {
                    self.board.move_piece(&position, s);
                }
            }
            self.next_turn();
            continue 'turn;
        }
    }

    fn get_opponent_color(&self) -> Color {
        match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }

    fn next_turn(&mut self) {
        self.turn = self.get_opponent_color();
    }

    fn wrong_color_prompt(&self) {
        match self.turn {
            Color::White => {
                println!("Du valde ei kvit brikke, men det er svart sin tur");
            },
            Color::Black => {
                println!("Du valde ei svart brikke, men det er kvit sin tur");
            }
        }
    }

    fn get_piece(&self) -> (u8, u8) {
        loop {
            print!("Vel ei brikke å flytte: ");
            io::stdout().flush().unwrap();
            match select_square() {
                Some(position) => {
                    match self.board.get_square_color(&position) {
                        Some(color) if color == self.turn => {
                            return position;
                        },
                        Some(_) => {
                            self.wrong_color_prompt();
                        },
                        None => {
                            println!("Det er inga brikke i feltet du valde");
                        }
                    }
                    continue
                }
                _ => continue
            }
        }
    }

    fn get_move(&self, position: &(u8, u8), mut legal_squares: HashSet<(u8, u8)>) -> (u8, u8) {
        loop {
            self.board.print_with_legal_moves(&position);
            print!("Vel eit felt å flytte til: ");
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

pub fn main() {
    let mut game = Game::new();
    game.play();
}