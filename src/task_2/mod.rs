use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_2::board::Board;

mod board;
pub mod piece;

pub fn main() {
    println!("Køyrer game::main() i oppgåve 2");
    game::main(Board::new())
}
