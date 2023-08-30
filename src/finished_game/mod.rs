use crate::finished_game::board::Board;
use crate::finished_game::board_contract::BoardContract;

mod board;
pub mod board_contract;
pub mod color;
pub mod game;
pub mod game_state;
pub mod piece;

pub fn main() {
    println!("Kjører game::main() i det ferdige spelet");
    game::main(Board::new())
}
