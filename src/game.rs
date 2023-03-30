use std::io;
use std::io::Write;
use crate::board_trait::Board;
use crate::enums::Color;
use crate::utils::{select_square};

pub fn main() {
    let mut board = Board::new();
    let mut turn = Color::White;

    'turn: loop {
        board.print();
        match turn {
            Color::White => println!("Kvit sin tur"),
            Color::Black => println!("Svart sin tur")
        }

        'select_piece: loop {
            print!("Vel ei brikke å flytte: ");
            io::stdout().flush().unwrap();
            match select_square() {
                Some(origin) => {
                    match board.get_square_color(&origin) {
                        Some(color) if color == turn => {

                            'select_target: loop {
                                board.print_with_legal_moves(&origin);
                                print!("Vel eit felt å flytte til: ");
                                io::stdout().flush().unwrap();
                                match select_square() {
                                    Some(target) => {
                                        match board.get_square_color(&target) {
                                            Some(color) if color != turn => {
                                                println!("{} fra {:?} fangar {} på {:?}", board.get_piece_name(&origin), origin, board.get_piece_name(&target), target);
                                                board.move_piece(origin, target);
                                            },
                                            Some(_color) => {
                                                println!("Feltet inneheld ei brikke med same farge");
                                                continue 'select_target
                                            },
                                            None => {
                                                println!("Flyttar {} fra {:?} til {:?}", board.get_piece_name(&origin), origin, target);
                                                board.move_piece(origin, target);

                                            }
                                        }
                                        match turn {
                                            Color::White => turn = Color::Black,
                                            Color::Black => turn = Color::White
                                        }
                                        continue 'turn
                                    },
                                    _ => continue 'select_target
                                }
                            }

                        },
                        Some(color) => {
                            match color {
                                Color::White => {
                                    println!("Du valde ei kvit brikke, men det er svart sin tur");
                                },
                                Color::Black => {
                                    println!("Du valde ei svart brikke, men det er kvit sin tur");
                                }
                            }
                        },
                        None => {
                            println!("Det er inga brikke i feltet du valde");
                        }
                    }
                    continue 'select_piece
                },
                _ => continue 'select_piece
            }
        }

    }
}