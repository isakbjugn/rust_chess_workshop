use std::any::type_name;
use std::collections::HashSet;
use std::io;
use std::io::{BufRead, Write};

use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::color::Color;
use crate::finished_game::game_state::GameState;
use crate::i18n::ChessTerm;
use crate::square::Square;

struct Game<'a> {
    board: Box<dyn BoardContract + 'a>,
    turn: Color,
    state: GameState,
}

impl<'a> Game<'a> {
    fn new(board: impl BoardContract + 'a) -> Self {
        Game { board: Box::new(board), turn: Color::White, state: GameState::Playing }
    }

    fn play(&mut self, input: &mut impl BufRead) {
        self.board.print(None);
        self.print_turn();
        'game: loop {
            if self.board.is_check(self.turn) {
                match self.board.is_checkmate(self.turn) {
                    false => println!("{} konge står i sjakk!", self.turn.print_capitalised()),
                    true => {
                        println!("{} konge er sjakkmatt!", self.turn.print_capitalised());
                        self.state = GameState::Checkmate(self.turn);
                        break 'game
                    }
                }
            }

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
                    let attacking = self.board.get_piece_type(&position).translate();
                    let attacked = self.board.get_piece_type(&position_to_move_to).translate();
                    println!("{} frå {} fangar {} på {}", attacking, position.as_string().unwrap(), attacked, position_to_move_to.as_string().unwrap());
                    self.board.move_piece(&position, position_to_move_to);
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

    fn warn_check(&self) {
        if self.board.is_check(self.turn) {
            println!("{} konge står i sjakk!", self.turn.print_capitalised());
        }
    }

    fn get_piece(&mut self, input: &mut impl BufRead) -> Option<(u8, u8)> {
        while self.state == GameState::Playing {
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
        while self.state == GameState::Playing {
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

        square.as_str().as_u8().ok()
    }

    pub fn exit_game(&mut self) {
        self.state = GameState::Quit;
    }
}

pub fn main(board: impl BoardContract) {
    let mut game = Game::new(board);
    game.play(&mut io::stdin().lock());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use crate::finished_game::board::Board;

    use super::*;

    #[test]
    fn move_a_piece() {
        let mut game = Game::new(Board::new());
        let input_data = "a2\na4\nx\n".as_bytes();
        let mut input = BufReader::new(input_data);

        game.play(&mut input);
    }
}