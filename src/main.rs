use crate::test_runner::all_tests_pass;

mod finished_game;
mod square;
mod test_runner;
mod task_1;
mod task_0;

fn main() {
    println!("Velkomen til Rust-workshop!");
    task_0::main();
    //task_1::main();
    if all_tests_pass() {
        println!("Alle tester kjører grønt! Starter sjakkspill:");
        //finished_game::main();
    } else {
        println!("Det var tester som kjørte rødt!");
    }
}
