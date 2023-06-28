use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_12::board::Board;

mod board;

pub fn main() {
    println!("Køyrer game::main() i oppgåve 12");
    game::main(Board::new())
}