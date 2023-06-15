use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_1::board::Board;

mod piece;
mod board;

/// # Oppgave 2
///
/// I denne oppgaven skal vi implementere de enkleste trekkene til bonden.
///
/// Du skal implementere funksjoner i filen [./piece/pawn.rs](./piece/pawn.rs). Der finner du
/// en oppgavebeskrivelse, og kommentarer som forklarer hva de ulike metodene skal gjøre.
///
/// Du finner også hint i [hint.md](./hint.md).
pub fn main() {
    println!("Køyrer game::main() i oppgåve 1");
    game::main(Board::new())
}
