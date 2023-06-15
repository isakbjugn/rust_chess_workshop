use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_6::board::Board;

mod piece;
mod board;

/// # Oppgave 6
/// 
/// I denne oppgaven skal vi implementere trekkene til løperen.
///
/// Du skal implementere funksjoner i filen [./piece/bishop.rs](./piece/bishop.rs). Der finner du
/// en oppgavebeskrivelse, og kommentarer som forklarer hva de ulike metodene skal gjøre.
///
/// Du finner også hint i [./hint.md](/hint.md).
pub fn main() {
    println!("Køyrer game::main() i oppgåve 6");
    game::main(Board::new())
}
