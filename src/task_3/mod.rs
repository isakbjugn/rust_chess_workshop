use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_3::board::Board;

mod piece;
mod board;

pub fn main() {
    println!("Køyrer game::main() i oppgåve 3");
    game::main(Board::new())
}


