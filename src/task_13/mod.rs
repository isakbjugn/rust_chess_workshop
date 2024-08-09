use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_13::board::Board;

mod board;
mod piece;

pub fn main() {
    println!("Køyrer game::main() i oppgåve 13");
    game::main(Board::new())
}