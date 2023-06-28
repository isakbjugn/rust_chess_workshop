use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_1::board::Board;

pub mod piece;
pub mod board;

pub fn main() {
    println!("Køyrer game::main() i oppgåve 1");
    game::main(Board::new())
}
