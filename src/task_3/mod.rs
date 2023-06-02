use crate::color::Color;
use crate::task_3::piece::Piece;

mod piece;
mod game;
mod board;

/// # Oppgave 3
/// 
/// I denne oppgaven skal vi implementere trekkene til springeren (hest).
///
/// Du skal implementere funksjoner i filen [./piece/knight.rs](./piece/knight.rs). Der finner du
/// en oppgavebeskrivelse, og kommentarer som forklarer hva de ulike metodene skal gjøre.
///
/// Du finner også hint i [./hint.md](/hint.md).
pub fn main() {
    println!("Køyrer game::main() i task_3");
    game::main()
}
