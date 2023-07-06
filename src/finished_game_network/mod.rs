use crate::finished_game::board_contract::BoardContract;
use crate::finished_game_network::board::Board;

mod board;
pub mod game;

pub fn main() {
    println!("Kjører game::main() i det ferdige spelet med nettverksfunksjonalitet");
    game::main(Board::new())
}