use std::env::args;

use crate::test_runner::highest_passing_test;

mod assertions;
mod finished_game;
mod finished_game_network;
mod i18n;
mod square;
mod task_0;
mod task_1;
mod task_10;
mod task_11;
mod task_12;
mod task_2;
mod task_3;
mod task_4;
mod task_5;
mod task_6;
mod task_7;
mod task_8;
mod task_9;
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
            "7" => task_7::main(),
            "8" => task_8::main(),
            "9" => task_9::main(),
            "10" => task_10::main(),
            "11" => task_11::main(),
            "12" => task_12::main(),
            "finished" => finished_game::main(),
            "finished_network" => finished_game_network::main(),
            _ => {
                println!("Ugyldig oppgåve");
                std::process::exit(1);
            }
        }
    } else {
        match highest_passing_test() {
            Some(task) => {
                println!("Tester til og med oppgåve {} køyrde grønt!", task);
                match task {
                    0 => task_1::main(),
                    _ => {
                        println!("Alle tester kjører grønt! Starter sjakkspill:");
                        finished_game::main()
                    }
                }
            }
            None => {
                println!("Alle tester køyrde rødt! Teiknar brett for oppgåve 0");
                task_0::main()
            }
        }
    }
}
