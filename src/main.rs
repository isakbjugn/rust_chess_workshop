use crate::test_runner::highest_passing_test;
use std::env::args;

mod assertions;
mod finished_game;
mod square;
mod task_0;
mod task_1;
mod task_2;
mod task_3;
mod task_4;
mod task_5;
mod task_6;
mod test_runner;

fn main() {
    println!("Velkomen til Rust-workshop!");

    if let Some(task) = args().nth(1) {
        match task.as_str() {
            "0" => task_0::main(),
            "1" => task_1::main(),
            "2" => task_2::main(),
            "3" => task_3::main(),
            "4" => task_4::main(),
            "5" => task_5::main(),
            "6" => task_6::main(),
            _ => {
                println!("Køyrer ferdig spel");
                finished_game::main();
            }
        }
    }

    if let Some(task) = highest_passing_test() {
        println!("Tester til og med oppgåve {} køyrde grønt!", task);
        match task {
            0 => task_1::main(),
            _ => {
                println!("Alle tester kjører grønt! Starter sjakkspill:");
                finished_game::main()
            }
        }
    }

    println!("Alle tester køyrde rødt! Teiknar brett for oppgåve 0");
    task_0::main()
}
