use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_7::board::Board;

mod piece;
mod board;

pub fn main() {
    println!("Køyrer game::main() i oppgåve 7");
    game::main(Board::new())
}
