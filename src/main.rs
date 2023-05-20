use crate::test_runner::all_tests_pass;

mod finished_game;
mod square;
mod test_runner;

fn main() {
    println!("Velkomen til Rust-workshop!");
    if all_tests_pass() {
        println!("Alle tester kjører grønt! Starter sjakkspill:");
        finished_game::main()
    } else {
        println!("Det var tester som kjøre rødt!");
    }
}
