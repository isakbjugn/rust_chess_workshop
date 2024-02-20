use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::board::Board;

pub mod game;

pub fn main() {
    println!("Kjører game::main() i det ferdige spelet med nettverksfunksjonalitet");
    game::main(Board::new())
}