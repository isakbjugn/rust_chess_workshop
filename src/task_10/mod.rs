use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::game;
use crate::task_10::board::Board;

mod board;

/// # Oppgave 10
///
/// Nå som vi har `is_check()` og klarer å ta hensyn til trekk som ville satt kongen i sjakk i
/// `get_legal_squares()`, kan vi nå faktisk implementere sjakkmatt!
///
/// Kongen er i sjakkmatt når han står i sjakk, og det ikke finnes noen trekk som kan forhindre at
/// kongen fortsatt står i sjakk. Du skal implementere `is_checkmate()` i `board.rs`.
///
/// Du finner også hint i [./hint.md](/hint.md).
pub fn main() {
    println!("Køyrer game::main() i oppgåve 10");
    game::main(Board::new())
}