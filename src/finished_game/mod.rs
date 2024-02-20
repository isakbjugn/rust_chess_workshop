use crate::finished_game::board::Board;
use crate::finished_game::board_contract::BoardContract;

pub mod piece;
pub mod board;
pub mod game;
pub mod color;
pub mod game_state;
pub mod board_contract;

pub fn main() {
    println!("Kjører game::main() i det ferdige spelet");
    game::main(Board::new())
}