pub(crate) mod piece;
mod board;
mod game;
pub mod color;

pub fn main() {
    println!("Kjører game::main() i finished_game");
    game::main()
}