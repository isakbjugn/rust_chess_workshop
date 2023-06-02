use crate::color::Color;
use crate::task_3::piece::Piece;

mod piece;
mod game;
mod board;

/// # Oppgave 3
/// 
/// I denne oppgaven skal vi implementere trekkene til springeren (hest).
pub fn main() {
    println!("Køyrer game::main() i task_3");
    game::main()
}
