use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_4::board::Board;

mod piece;
mod board;

/// # Oppgave 4
/// 
/// I denne oppgaven skal vi implementere trekkene til kongen.
///
/// Du skal implementere funksjoner i filen [./piece/king.rs](./piece/king.rs). Der finner du
/// en oppgavebeskrivelse, og kommentarer som forklarer hva de ulike metodene skal gjøre.
///
/// Du finner også hint i [./hint.md](/hint.md).
pub fn main() {
    println!("Køyrer game::main() i oppgåve 4");
    game::main(Board::new())
}
