use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_3::board::Board;

mod piece;
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
    println!("Køyrer game::main() i oppgåve 3");
    game::main(Board::new())
}
