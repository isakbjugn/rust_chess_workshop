pub(crate) mod piece;
mod board;
mod game;
pub mod color;

pub fn main() {
    println!("Kjører game::main() i det ferdige spelet");
    game::main()
}