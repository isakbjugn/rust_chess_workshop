use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_8::board::Board;

mod piece;
mod board;

pub fn main() {
    println!("Køyrer game::main() i oppgåve 8");
    game::main(Board::new())
}
