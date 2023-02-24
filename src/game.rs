use std::io;
use std::io::Write;
use crate::board::Board;
use crate::utils::{select_square};

pub fn main() {
    let mut board = Board::new();

    loop {
        board.print();

        print!("Vel ei brikke å flytte: ");
        io::stdout().flush().unwrap();
        match select_square() {
            Some(origin) if board.position_holds_piece(origin) => {
                print!("Vel eit felt å flytte til: ");
                io::stdout().flush().unwrap();
                match select_square() {
                    Some(square) => {
                        if board.position_holds_piece(square) {
                            println!("Feltet er allereie tatt")
                        }
                        println!("Flyttar {} fra {:?} til {:?}", board.get_piece_type(origin).unwrap(), origin, square);
                        board.move_piece(origin, square);
                    },
                    _ => continue
                }
            },
            _ => continue
        }
    }
}